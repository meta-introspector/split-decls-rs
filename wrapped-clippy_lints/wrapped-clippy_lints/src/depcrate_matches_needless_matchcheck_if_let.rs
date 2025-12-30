// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_needless_matchcheck_if_let {
() => {
// Module: crate::matches::needless_match
// Provides: {"check_if_let"}
// Dependencies: {}
# [doc = " Check for nop `if let` expression that assembled as unnecessary match"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " if let Some(a) = option {"] # [doc = "     Some(a)"] # [doc = " } else {"] # [doc = "     None"] # [doc = " }"] # [doc = " ```"] # [doc = " OR"] # [doc = " ```rust,ignore"] # [doc = " if let SomeEnum::A = some_enum {"] # [doc = "     SomeEnum::A"] # [doc = " } else if let SomeEnum::B = some_enum {"] # [doc = "     SomeEnum::B"] # [doc = " } else {"] # [doc = "     some_enum"] # [doc = " }"] # [doc = " ```"] pub (crate) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , ex : & Expr < '_ > , if_let : & higher :: IfLet < 'tcx >) { if ! is_else_clause (cx . tcx , ex) && expr_ty_matches_p_ty (cx , if_let . let_expr , ex) && check_if_let_inner (cx , if_let) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , NEEDLESS_MATCH , ex . span , "this if-let expression is unnecessary" , "replace it with" , snippet_with_applicability (cx , if_let . let_expr . span , ".." , & mut applicability) . to_string () , applicability ,) ; } }
};
}
