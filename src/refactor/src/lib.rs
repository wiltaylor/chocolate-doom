#[macro_use]
extern crate lazy_mut;

mod d_event;
mod i_main;

#[no_mangle]
pub extern "C" fn test_rust() {
    println!("Testing rust");
}