// Generated macro for impl_286 (impl)
macro_rules! Depcrate_error_kindimpl_286 {
() => {
// Module: crate::error::kind
// Provides: {"impl_286"}
// Dependencies: {}
impl ErrorKind { # [doc = " Deeply counts the number of errors this item represents."] pub fn len (& self) -> usize { if let ErrorKind :: Multiple (ref items) = * self { items . iter () . map (Error :: len) . sum () } else { 1 } } }
};
}
