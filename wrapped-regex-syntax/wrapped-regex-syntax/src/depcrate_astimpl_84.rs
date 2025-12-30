// Generated macro for impl_84 (impl)
macro_rules! Depcrate_astimpl_84 {
() => {
// Module: crate::ast
// Provides: {"impl_84"}
// Dependencies: {}
impl Concat { # [doc = " Return this concatenation as an AST."] # [doc = ""] # [doc = " If this alternation contains zero ASTs, then `Ast::empty` is returned."] # [doc = " If this alternation contains exactly 1 AST, then the corresponding AST"] # [doc = " is returned. Otherwise, `Ast::concat` is returned."] pub fn into_ast (mut self) -> Ast { match self . asts . len () { 0 => Ast :: empty (self . span) , 1 => self . asts . pop () . unwrap () , _ => Ast :: concat (self) , } } }
};
}
