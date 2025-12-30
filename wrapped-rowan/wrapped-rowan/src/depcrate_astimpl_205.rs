// Generated macro for impl_205 (impl)
macro_rules! Depcrate_astimpl_205 {
() => {
// Module: crate::ast
// Provides: {"impl_205"}
// Dependencies: {}
impl < N : AstNode > AstChildren < N > { fn new (parent : & SyntaxNode < N :: Language >) -> Self { AstChildren { inner : parent . children () , ph : PhantomData } } }
};
}
