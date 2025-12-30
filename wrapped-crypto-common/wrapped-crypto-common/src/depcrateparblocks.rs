// Generated macro for ParBlocks (type)
macro_rules! DepcrateParBlocks {
() => {
// Module: crate
// Provides: {"ParBlocks"}
// Dependencies: {}
# [doc = " Parallel blocks on which [`ParBlocksSizeUser`] implementors operate."] pub type ParBlocks < T > = Array < Block < T > , < T as ParBlocksSizeUser > :: ParBlocksSize > ;
};
}
