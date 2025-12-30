// Generated macro for impl_100 (impl)
macro_rules! Depcrate_xof_fixedimpl_100 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : ExtendableOutputReset , S : ArraySize > FixedOutputReset for XofFixedWrapper < T , S > { fn finalize_into_reset (& mut self , out : & mut crypto_common :: Output < Self >) { self . hash . finalize_xof_reset_into (out) ; } }
};
}
