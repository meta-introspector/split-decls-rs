// Generated macro for initialize (function)
macro_rules! Depcrateinitialize {
() => {
// Module: crate
// Provides: {"initialize"}
// Dependencies: {}
# [doc = " Takes a shared reference to a lazy static and initializes"] # [doc = " it if it has not been already."] # [doc = ""] # [doc = " This can be used to control the initialization point of a lazy static."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use lazy_static::lazy_static;"] # [doc = ""] # [doc = " lazy_static! {"] # [doc = "     static ref BUFFER: Vec<u8> = (0..255).collect();"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     lazy_static::initialize(&BUFFER);"] # [doc = ""] # [doc = "     // ..."] # [doc = "     work_with_initialized_data(&BUFFER);"] # [doc = " }"] # [doc = " # fn work_with_initialized_data(_: &[u8]) {}"] # [doc = " ```"] pub fn initialize < T : LazyStatic > (lazy : & T) { LazyStatic :: initialize (lazy) ; }
};
}
