// Generated macro for impl_99 (impl)
macro_rules! Depcrate_xof_fixedimpl_99 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : ExtendableOutput + Update , S : ArraySize > FixedOutput for XofFixedWrapper < T , S > { fn finalize_into (self , out : & mut crypto_common :: Output < Self >) { self . hash . finalize_xof_into (out) ; } }
};
}
