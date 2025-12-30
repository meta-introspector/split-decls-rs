// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > ToHexIter < T > { # [doc = " Create new hex-converting iterator."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main () {"] # [doc = "     let bytes = vec![1, 2, 3, 4];"] # [doc = "     let iter = rustc_hex::ToHexIter::new(bytes.iter());"] # [doc = "     println!(\"{}\", iter.collect::<String>());"] # [doc = " }"] # [doc = " ```"] pub fn new (inner : T) -> Self { Self { live : None , inner , } } }
};
}
