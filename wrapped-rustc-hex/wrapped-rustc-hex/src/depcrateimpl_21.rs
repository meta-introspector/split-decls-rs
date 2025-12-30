// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > FromHexIter < 'a > { # [doc = " Create new hex-decoding iterator."] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main () {"] # [doc = "     let s = \"ff0102\";"] # [doc = "     let iter = rustc_hex::FromHexIter::new(s);"] # [doc = "     println!(\"{:?}\", iter.collect::<Vec<_>>());"] # [doc = " }"] # [doc = " ```"] pub fn new (inner : & 'a str) -> Self { let iter = inner . bytes () . enumerate () ; Self { err : false , inner , iter , } } }
};
}
