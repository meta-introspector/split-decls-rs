// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_saturating_arithmeticcheck {
() => {
// Module: crate::methods::manual_saturating_arithmetic
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , arith_lhs : & hir :: Expr < '_ > , arith_rhs : & hir :: Expr < '_ > , unwrap_arg : & hir :: Expr < '_ > , arith : & str ,) { let ty = cx . typeck_results () . expr_ty (arith_lhs) ; if ! ty . is_integral () { return ; } let Some (mm) = is_min_or_max (cx , unwrap_arg) else { return ; } ; if ty . is_signed () { use self :: MinMax :: { Max , Min } ; use self :: Sign :: { Neg , Pos } ; let Some (sign) = lit_sign (arith_rhs) else { return ; } ; match (arith , sign , mm) { ("add" , Pos , Max) | ("add" , Neg , Min) | ("sub" , Neg , Max) | ("sub" , Pos , Min) => () , _ => return , } } else { match (mm , arith) { (MinMax :: Max , "add" | "mul") | (MinMax :: Min , "sub") => () , _ => return , } } let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , super :: MANUAL_SATURATING_ARITHMETIC , expr . span , "manual saturating arithmetic" , format ! ("consider using `saturating_{arith}`") , format ! ("{}.saturating_{arith}({})" , snippet_with_applicability (cx , arith_lhs . span , ".." , & mut applicability) , snippet_with_applicability (cx , arith_rhs . span , ".." , & mut applicability) ,) , applicability ,) ; }
};
}
