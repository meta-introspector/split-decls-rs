// Generated macro for Buffer (type)
macro_rules! Depcrate_block_apiBuffer {
() => {
// Module: crate::block_api
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " Buffer type used by type which implements [`BufferKindUser`]."] pub type Buffer < S > = BlockBuffer < < S as BlockSizeUser > :: BlockSize , < S as BufferKindUser > :: BufferKind > ;
};
}
