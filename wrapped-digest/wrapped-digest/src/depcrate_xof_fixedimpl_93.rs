// Generated macro for impl_93 (impl)
macro_rules! Depcrate_xof_fixedimpl_93 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_93"}
// Dependencies: {}
impl < T : ExtendableOutput + BlockSizeUser , S : ArraySize > BlockSizeUser for XofFixedWrapper < T , S > { type BlockSize = T :: BlockSize ; }
};
}
