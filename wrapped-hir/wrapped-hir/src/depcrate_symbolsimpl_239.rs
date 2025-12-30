// Generated macro for impl_239 (impl)
macro_rules! Depcrate_symbolsimpl_239 {
() => {
// Module: crate::symbols
// Provides: {"impl_239"}
// Dependencies: {}
impl DeclarationLocation { pub fn syntax < DB : HirDatabase > (& self , sema : & Semantics < '_ , DB >) -> SyntaxNode { let root = sema . parse_or_expand (self . hir_file_id) ; self . ptr . to_node (& root) } }
};
}
