// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl FromHex for str { # [doc = " Convert any hexadecimal encoded string (literal, `@`, `&`, or `~`)"] # [doc = " to the byte values it encodes."] # [doc = ""] # [doc = " You can use the `String::from_utf8` function to turn a"] # [doc = " `Vec<u8>` into a string with characters corresponding to those values."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This converts a string literal to hexadecimal and back."] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate rustc_hex;"] # [doc = " use rustc_hex::{FromHex, ToHex};"] # [doc = ""] # [doc = " fn main () {"] # [doc = "     let hello_str: String = \"Hello, World\".as_bytes().to_hex();"] # [doc = "     println!(\"{}\", hello_str);"] # [doc = "     let bytes: Vec<u8> = hello_str.from_hex().unwrap();"] # [doc = "     println!(\"{:?}\", bytes);"] # [doc = "     let result_str = String::from_utf8(bytes).unwrap();"] # [doc = "     println!(\"{}\", result_str);"] # [doc = " }"] # [doc = " ```"] fn from_hex < T : FromIterator < u8 > > (& self) -> Result < T , FromHexError > { FromHexIter :: new (self) . collect () } }
};
}
