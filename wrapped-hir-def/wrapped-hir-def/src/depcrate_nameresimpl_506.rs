// Generated macro for impl_506 (impl)
macro_rules! Depcrate_nameresimpl_506 {
() => {
// Module: crate::nameres
// Provides: {"impl_506"}
// Dependencies: {}
impl ModuleSource { pub fn node (& self) -> SyntaxNode { match self { ModuleSource :: SourceFile (it) => it . syntax () . clone () , ModuleSource :: Module (it) => it . syntax () . clone () , ModuleSource :: BlockExpr (it) => it . syntax () . clone () , } } }
};
}
