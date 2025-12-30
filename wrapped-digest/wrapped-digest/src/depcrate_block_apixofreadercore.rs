// Generated macro for XofReaderCore (trait)
macro_rules! Depcrate_block_apiXofReaderCore {
() => {
// Module: crate::block_api
// Provides: {"XofReaderCore"}
// Dependencies: {}
# [doc = " Core reader trait for extendable-output function (XOF) result."] pub trait XofReaderCore : BlockSizeUser { # [doc = " Read next XOF block."] fn read_block (& mut self) -> Block < Self > ; }
};
}
