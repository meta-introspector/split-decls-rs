// Generated macro for impl_23 (impl)
macro_rules! Depcrate_grammarimpl_23 {
() => {
// Module: crate::grammar
// Provides: {"impl_23"}
// Dependencies: {}
impl BlockLike { fn is_block (self) -> bool { self == BlockLike :: Block } fn is_blocklike (kind : SyntaxKind) -> bool { matches ! (kind , BLOCK_EXPR | IF_EXPR | WHILE_EXPR | FOR_EXPR | LOOP_EXPR | MATCH_EXPR) } }
};
}
