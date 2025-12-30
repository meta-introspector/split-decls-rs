// Generated macro for check (function)
macro_rules! Depcrate_methods_is_digit_ascii_radixcheck {
() => {
// Module: crate::methods::is_digit_ascii_radix
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , self_arg : & 'tcx Expr < '_ > , radix : & 'tcx Expr < '_ > , msrv : Msrv ,) { if ! cx . typeck_results () . expr_ty_adjusted (self_arg) . peel_refs () . is_char () { return ; } if let Some (radix_val) = ConstEvalCtxt :: new (cx) . eval_full_int (radix , expr . span . ctxt ()) { let (num , replacement) = match radix_val { FullInt :: S (10) | FullInt :: U (10) => (10 , "is_ascii_digit") , FullInt :: S (16) | FullInt :: U (16) => (16 , "is_ascii_hexdigit") , _ => return , } ; let mut applicability = Applicability :: MachineApplicable ; if ! msrv . meets (cx , msrvs :: IS_ASCII_DIGIT) { return ; } span_lint_and_sugg (cx , IS_DIGIT_ASCII_RADIX , expr . span , format ! ("use of `char::is_digit` with literal radix of {num}") , "try" , format ! ("{}.{replacement}()" , snippet_with_applicability (cx , self_arg . span , ".." , & mut applicability)) , applicability ,) ; } }
};
}
