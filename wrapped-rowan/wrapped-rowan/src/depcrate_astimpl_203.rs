// Generated macro for impl_203 (impl)
macro_rules! Depcrate_astimpl_203 {
() => {
// Module: crate::ast
// Provides: {"impl_203"}
// Dependencies: {}
impl < N : AstNode > From < AstPtr < N > > for SyntaxNodePtr < N :: Language > { fn from (ptr : AstPtr < N >) -> SyntaxNodePtr < N :: Language > { ptr . raw } }
};
}
