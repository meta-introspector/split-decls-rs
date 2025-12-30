// Generated macro for impl_512 (impl)
macro_rules! Depcrate_nameresimpl_512 {
() => {
// Module: crate::nameres
// Provides: {"impl_512"}
// Dependencies: {}
impl ModuleSource { pub fn node (& self) -> SyntaxNode { match self { ModuleSource :: SourceFile (it) => it . syntax () . clone () , ModuleSource :: Module (it) => it . syntax () . clone () , ModuleSource :: BlockExpr (it) => it . syntax () . clone () , } } }
};
}
