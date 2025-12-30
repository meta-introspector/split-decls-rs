// Generated macro for BufferKindUser (trait)
macro_rules! Depcrate_block_apiBufferKindUser {
() => {
// Module: crate::block_api
// Provides: {"BufferKindUser"}
// Dependencies: {}
# [doc = " Types which use [`BlockBuffer`] functionality."] pub trait BufferKindUser : BlockSizeUser { # [doc = " Block buffer kind over which type operates."] type BufferKind : BufferKind ; }
};
}
