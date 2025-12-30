// Generated macro for EagerHash (trait)
macro_rules! Depcrate_block_apiEagerHash {
() => {
// Module: crate::block_api
// Provides: {"EagerHash"}
// Dependencies: {}
# [doc = " Trait implemented by eager hashes which expose their block-level core."] pub trait EagerHash : BlockSizeUser + Digest { # [doc = " Block-level core type of the hash."] type Core : HashMarker + UpdateCore + FixedOutputCore + BlockSizeUser < BlockSize = < Self as BlockSizeUser > :: BlockSize > + BufferKindUser < BufferKind = Eager > + Default + Clone ; }
};
}
