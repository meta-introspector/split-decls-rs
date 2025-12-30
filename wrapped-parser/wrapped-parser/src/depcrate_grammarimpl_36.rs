// Generated macro for impl_36 (impl)
macro_rules! Depcrate_grammarimpl_36 {
() => {
// Module: crate::grammar
// Provides: {"impl_36"}
// Dependencies: {}
impl BlockLike { fn is_block (self) -> bool { self == BlockLike :: Block } fn is_blocklike (kind : SyntaxKind) -> bool { matches ! (kind , BLOCK_EXPR | IF_EXPR | WHILE_EXPR | FOR_EXPR | LOOP_EXPR | MATCH_EXPR) } }
};
}
