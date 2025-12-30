// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl ToHex for [u8] { # [doc = " Turn a slice of `u8` bytes into a hexadecimal string."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rustc_hex::ToHex;"] # [doc = ""] # [doc = " fn main () {"] # [doc = "     let str: String = [52,32].to_hex();"] # [doc = "     println!(\"{}\", str);"] # [doc = " }"] # [doc = " ```"] fn to_hex < T : FromIterator < char > > (& self) -> T { ToHexIter :: new (self . iter ()) . collect () } }
};
}
