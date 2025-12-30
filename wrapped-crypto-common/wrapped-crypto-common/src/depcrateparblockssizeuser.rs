// Generated macro for ParBlocksSizeUser (trait)
macro_rules! DepcrateParBlocksSizeUser {
() => {
// Module: crate
// Provides: {"ParBlocksSizeUser"}
// Dependencies: {}
# [doc = " Types which can process blocks in parallel."] pub trait ParBlocksSizeUser : BlockSizeUser { # [doc = " Number of blocks which can be processed in parallel."] type ParBlocksSize : ArraySize ; }
};
}
