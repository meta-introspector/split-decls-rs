// Generated macro for impl_195 (impl)
macro_rules! Depcrate_hir_literalimpl_195 {
() => {
// Module: crate::hir::literal
// Provides: {"impl_195"}
// Dependencies: {}
impl From < char > for Literal { fn from (ch : char) -> Literal { use alloc :: string :: ToString ; Literal :: exact (ch . encode_utf8 (& mut [0 ; 4]) . to_string ()) } }
};
}
