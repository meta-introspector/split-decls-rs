// Generated macro for impl_3136 (impl)
macro_rules! Depcrate_items_after_statementsimpl_3136 {
() => {
// Module: crate::items_after_statements
// Provides: {"impl_3136"}
// Dependencies: {}
impl LateLintPass < '_ > for ItemsAfterStatements { fn check_block (& mut self , cx : & LateContext < '_ > , block : & Block < '_ >) { if block . stmts . len () > 1 { let ctxt = block . span . ctxt () ; let mut in_external = None ; block . stmts . iter () . skip_while (| stmt | matches ! (stmt . kind , StmtKind :: Item (..))) . filter_map (| stmt | match stmt . kind { StmtKind :: Item (id) => Some (cx . tcx . hir_item (id)) , _ => None , }) . filter (| item | ! matches ! (item . kind , ItemKind :: Macro (..))) . take_while (| item | item . span . ctxt () == ctxt) . for_each (| item | { if ! * in_external . get_or_insert_with (| | block . span . in_external_macro (cx . sess () . source_map ())) { span_lint_hir (cx , ITEMS_AFTER_STATEMENTS , item . hir_id () , item . span , "adding items after statements is confusing, since items exist from the \
                                start of the scope" ,) ; } }) ; } } }
};
}
