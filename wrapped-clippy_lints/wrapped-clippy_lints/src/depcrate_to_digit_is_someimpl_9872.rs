// Generated macro for impl_9872 (impl)
macro_rules! Depcrate_to_digit_is_someimpl_9872 {
() => {
// Module: crate::to_digit_is_some
// Provides: {"impl_9872"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ToDigitIsSome { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if let hir :: ExprKind :: MethodCall (is_some_path , to_digit_expr , [] , _) = & expr . kind && is_some_path . ident . name == sym :: is_some { let match_result = match to_digit_expr . kind { hir :: ExprKind :: MethodCall (to_digits_path , char_arg , [radix_arg] , _) => { if to_digits_path . ident . name == sym :: to_digit && cx . typeck_results () . expr_ty_adjusted (char_arg) . is_char () { Some ((true , char_arg , radix_arg)) } else { None } } , hir :: ExprKind :: Call (to_digits_call , [char_arg , radix_arg]) => { if is_path_diagnostic_item (cx , to_digits_call , sym :: char_to_digit) { Some ((false , char_arg , radix_arg)) } else { None } } , _ => None , } ; if let Some ((is_method_call , char_arg , radix_arg)) = match_result && (! is_in_const_context (cx) || self . msrv . meets (cx , msrvs :: CONST_CHAR_IS_DIGIT)) { let mut applicability = Applicability :: MachineApplicable ; let char_arg_snip = snippet_with_applicability (cx , char_arg . span , "_" , & mut applicability) ; let radix_snip = snippet_with_applicability (cx , radix_arg . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , TO_DIGIT_IS_SOME , expr . span , "use of `.to_digit(..).is_some()`" , "try" , if is_method_call { format ! ("{char_arg_snip}.is_digit({radix_snip})") } else { format ! ("char::is_digit({char_arg_snip}, {radix_snip})") } , applicability ,) ; } } } }
};
}
