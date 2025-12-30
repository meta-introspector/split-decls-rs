// Generated macro for impl_26 (impl)
macro_rules! Depcrate_ast_data_nested_metaimpl_26 {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"impl_26"}
// Dependencies: {}
impl syn :: parse :: Parse for NestedMeta { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { if input . peek (syn :: Lit) && ! (input . peek (syn :: LitBool) && input . peek2 (syn :: Token ! [=])) { input . parse () . map (Self :: Lit) } else if input . peek (syn :: Ident :: peek_any) { let path = parse_meta_path (input) ? ; parse_meta_after_path (path , input) . map (Self :: Meta) } else { Err (input . error ("expected identifier or literal")) } } }
};
}
