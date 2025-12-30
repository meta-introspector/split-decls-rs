// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_checked_conversionsimpl_1119 {
() => {
// Module: crate::checked_conversions
// Provides: {"impl_1119"}
// Dependencies: {}
impl LateLintPass < '_ > for CheckedConversions { fn check_expr (& mut self , cx : & LateContext < '_ > , item : & Expr < '_ >) { if let ExprKind :: Binary (op , lhs , rhs) = item . kind && let (lt1 , gt1 , op2) = match op . node { BinOpKind :: Le => (lhs , rhs , None) , BinOpKind :: Ge => (rhs , lhs , None) , BinOpKind :: And if let ExprKind :: Binary (op1 , lhs1 , rhs1) = lhs . kind && let ExprKind :: Binary (op2 , lhs2 , rhs2) = rhs . kind && let Some ((lt1 , gt1)) = read_le_ge (op1 . node , lhs1 , rhs1) && let Some ((lt2 , gt2)) = read_le_ge (op2 . node , lhs2 , rhs2) => { (lt1 , gt1 , Some ((lt2 , gt2))) } , _ => return , } && ! item . span . in_external_macro (cx . sess () . source_map ()) && ! is_in_const_context (cx) && let Some (cv) = match op2 { None => check_upper_bound (lt1 , gt1) . filter (| cv | cv . cvt == ConversionType :: FromUnsigned) , Some ((lt2 , gt2)) => { let upper_lower = | lt1 , gt1 , lt2 , gt2 | { check_upper_bound (lt1 , gt1) . zip (check_lower_bound (lt2 , gt2)) . and_then (| (l , r) | l . combine (r , cx)) } ; upper_lower (lt1 , gt1 , lt2 , gt2) . or_else (| | upper_lower (lt2 , gt2 , lt1 , gt1)) } , } && let Some (to_type) = cv . to_type && self . msrv . meets (cx , msrvs :: TRY_FROM) { let mut applicability = Applicability :: MachineApplicable ; let snippet = snippet_with_applicability (cx , cv . expr_to_cast . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , CHECKED_CONVERSIONS , item . span , "checked cast can be simplified" , "try" , format ! ("{to_type}::try_from({snippet}).is_ok()") , applicability ,) ; } } }
};
}
