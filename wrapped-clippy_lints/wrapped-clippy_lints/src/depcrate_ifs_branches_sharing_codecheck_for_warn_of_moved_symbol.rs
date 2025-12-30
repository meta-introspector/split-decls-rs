// Generated macro for check_for_warn_of_moved_symbol (function)
macro_rules! Depcrate_ifs_branches_sharing_codecheck_for_warn_of_moved_symbol {
() => {
// Module: crate::ifs::branches_sharing_code
// Provides: {"check_for_warn_of_moved_symbol"}
// Dependencies: {}
fn check_for_warn_of_moved_symbol (cx : & LateContext < '_ > , symbols : & [(HirId , Symbol)] , if_expr : & Expr < '_ >) -> bool { get_enclosing_block (cx , if_expr . hir_id) . is_some_and (| block | { let ignore_span = block . span . shrink_to_lo () . to (if_expr . span) ; symbols . iter () . filter (| & & (_ , name) | ! name . as_str () . starts_with ('_')) . any (| & (_ , name) | { let mut walker = ContainsName { name , cx } ; let mut res = block . stmts . iter () . filter (| stmt | ! ignore_span . overlaps (stmt . span)) . try_for_each (| stmt | intravisit :: walk_stmt (& mut walker , stmt)) ; if let Some (expr) = block . expr && res . is_continue () { res = intravisit :: walk_expr (& mut walker , expr) ; } res . is_break () }) }) }
};
}
