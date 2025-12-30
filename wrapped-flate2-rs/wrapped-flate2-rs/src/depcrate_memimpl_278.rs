// Generated macro for impl_278 (impl)
macro_rules! Depcrate_memimpl_278 {
() => {
// Module: crate::mem
// Provides: {"impl_278"}
// Dependencies: {}
impl DecompressError { # [doc = " Retrieve the implementation's message about why the operation failed, if one exists."] pub fn message (& self) -> Option < & str > { match & self . 0 { DecompressErrorInner :: General { msg } => msg . get () , _ => None , } } }
};
}
