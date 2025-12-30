// Generated macro for Block (enum)
macro_rules! Depcrate_compilerBlock {
() => {
// Module: crate::compiler
// Provides: {"Block"}
// Dependencies: {}
# [doc = " The compiler keeps a stack of the open blocks so that it can ensure that blocks are closed in"] # [doc = " the right order. The Block type is a simple enumeration of the kinds of blocks that could be"] # [doc = " open. It may contain the instruction index corresponding to the start of the block."] enum Block { Branch (usize) , For (usize) , With , }
};
}
