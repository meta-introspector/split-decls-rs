// Generated macro for lowercase_byte (function)
macro_rules! Depcrate_traitslowercase_byte {
() => {
// Module: crate::traits
// Provides: {"lowercase_byte"}
// Dependencies: {}
fn lowercase_byte (c : u8) -> u8 { match c { b'A' ..= b'Z' => c - b'A' + b'a' , _ => c , } }
};
}
