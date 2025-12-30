// Generated macro for impl_102 (impl)
macro_rules! Depcrate_xof_fixedimpl_102 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_102"}
// Dependencies: {}
impl < T : ExtendableOutputReset , S : ArraySize > ExtendableOutputReset for XofFixedWrapper < T , S > { fn finalize_xof_reset (& mut self) -> Self :: Reader { self . hash . finalize_xof_reset () } }
};
}
