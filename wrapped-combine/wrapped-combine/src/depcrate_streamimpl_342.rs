// Generated macro for impl_342 (impl)
macro_rules! Depcrate_streamimpl_342 {
() => {
// Module: crate::stream
// Provides: {"impl_342"}
// Dependencies: {}
impl < T > PointerOffset < T > where T : ? Sized , { pub fn new (offset : usize) -> Self { PointerOffset (offset , PhantomData) } # [doc = " Converts the pointer-based position into an indexed position."] # [doc = ""] # [doc = " ```rust"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let text = \"b\";"] # [doc = " let err = token('a').easy_parse(text).unwrap_err();"] # [doc = " assert_eq!(err.position.0, text.as_ptr() as usize);"] # [doc = " assert_eq!(err.map_position(|p| p.translate_position(text)).position, 0);"] # [doc = " # }"] # [doc = " ```"] pub fn translate_position (mut self , initial_slice : & T) -> usize { self . 0 -= initial_slice as * const T as * const () as usize ; self . 0 } }
};
}
