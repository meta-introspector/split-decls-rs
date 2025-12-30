// Generated macro for check_len (function)
macro_rules! Depcrate_len_zerocheck_len {
() => {
// Module: crate::len_zero
// Provides: {"check_len"}
// Dependencies: {}
fn check_len (cx : & LateContext < '_ > , span : Span , method_name : Symbol , receiver : & Expr < '_ > , lit : & LitKind , op : & str , compare_to : u32 ,) { if let LitKind :: Int (lit , _) = * lit { if lit != u128 :: from (compare_to) { return ; } if method_name == sym :: len && has_is_empty (cx , receiver) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , LEN_ZERO , span , format ! ("length comparison to {}" , if compare_to == 0 { "zero" } else { "one" }) , format ! ("using `{op}is_empty` is clearer and more explicit") , format ! ("{op}{}.is_empty()" , snippet_with_context (cx , receiver . span , span . ctxt () , "_" , & mut applicability) . 0 ,) , applicability ,) ; } } }
};
}
