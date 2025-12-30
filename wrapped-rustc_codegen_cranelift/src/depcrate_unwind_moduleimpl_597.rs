// Generated macro for impl_597 (impl)
macro_rules! Depcrate_unwind_moduleimpl_597 {
() => {
// Module: crate::unwind_module
// Provides: {"impl_597"}
// Dependencies: {}
impl UnwindModule < ObjectModule > { pub (crate) fn finish (self) -> ObjectProduct { let mut product = self . module . finish () ; self . unwind_context . emit (& mut product) ; product } }
};
}
