// Generated macro for impl_53 (impl)
macro_rules! Depcrate_block_apiimpl_53 {
() => {
// Module: crate::block_api
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > EagerHash for T where T : CoreProxy + BlockSizeUser + Digest , < T as CoreProxy > :: Core : HashMarker + UpdateCore + FixedOutputCore + BlockSizeUser < BlockSize = < Self as BlockSizeUser > :: BlockSize > + BufferKindUser < BufferKind = Eager > + Default + Clone , { type Core = T :: Core ; }
};
}
