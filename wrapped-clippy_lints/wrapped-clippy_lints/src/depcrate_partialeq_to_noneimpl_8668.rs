// Generated macro for impl_8668 (impl)
macro_rules! Depcrate_partialeq_to_noneimpl_8668 {
() => {
// Module: crate::partialeq_to_none
// Provides: {"impl_8668"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PartialeqToNone { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if e . span . from_expansion () { return ; } let is_ty_option = | expr : & Expr < '_ > | is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty (expr) . peel_refs () , sym :: Option) ; let is_none_ctor = | expr : & Expr < '_ > | { ! expr . span . from_expansion () && is_res_lang_ctor (cx , path_res (cx , peel_hir_expr_refs (expr) . 0) , LangItem :: OptionNone) } ; let mut applicability = Applicability :: MachineApplicable ; if let ExprKind :: Binary (op , left_side , right_side) = e . kind { let is_eq = match op . node { BinOpKind :: Eq => true , BinOpKind :: Ne => false , _ => return , } ; let scrutinee = match (is_none_ctor (left_side) && is_ty_option (right_side) , is_none_ctor (right_side) && is_ty_option (left_side) ,) { (true , false) => right_side , (false , true) => left_side , _ => return , } ; let sugg = format ! ("{}.{}" , sugg :: Sugg :: hir_with_applicability (cx , peel_ref_operators (cx , scrutinee) , ".." , & mut applicability) . maybe_paren () , if is_eq { "is_none()" } else { "is_some()" }) ; span_lint_and_sugg (cx , PARTIALEQ_TO_NONE , e . span , "binary comparison to literal `Option::None`" , if is_eq { "use `Option::is_none()` instead" } else { "use `Option::is_some()` instead" } , sugg , applicability ,) ; } } }
};
}
