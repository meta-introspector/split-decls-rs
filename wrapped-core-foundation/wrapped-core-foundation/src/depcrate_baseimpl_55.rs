// Generated macro for impl_55 (impl)
macro_rules! Depcrate_baseimpl_55 {
() => {
// Module: crate::base
// Provides: {"impl_55"}
// Dependencies: {}
impl CFIndexConvertible for usize { # [inline] fn to_CFIndex (self) -> CFIndex { if self > (CFIndex :: MAX as usize) { panic ! ("value out of range") } self as CFIndex } }
};
}
