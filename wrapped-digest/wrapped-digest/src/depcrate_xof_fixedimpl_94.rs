// Generated macro for impl_94 (impl)
macro_rules! Depcrate_xof_fixedimpl_94 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_94"}
// Dependencies: {}
impl < T : ExtendableOutput + KeySizeUser , S : ArraySize > KeySizeUser for XofFixedWrapper < T , S > { type KeySize = T :: KeySize ; }
};
}
