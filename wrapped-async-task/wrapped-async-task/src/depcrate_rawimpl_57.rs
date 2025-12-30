// Generated macro for impl_57 (impl)
macro_rules! Depcrate_rawimpl_57 {
() => {
// Module: crate::raw
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > PointerPolyfill for * const T { # [inline] unsafe fn add_byte (self , size : usize) -> Self { (self . cast :: < u8 > () . add (size)) . cast :: < T > () } }
};
}
