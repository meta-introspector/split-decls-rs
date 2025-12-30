// Generated macro for span_ineffective_operation (function)
macro_rules! Depcrate_operators_identity_opspan_ineffective_operation {
() => {
// Module: crate::operators::identity_op
// Provides: {"span_ineffective_operation"}
// Dependencies: {}
fn span_ineffective_operation (cx : & LateContext < '_ > , span : Span , arg : Span , parens : Parens , is_ref_coerced_to_val : bool ,) { let mut applicability = Applicability :: MachineApplicable ; let expr_snippet = snippet_with_applicability (cx , arg , ".." , & mut applicability) ; let expr_snippet = if is_ref_coerced_to_val { format ! ("*{expr_snippet}") } else { expr_snippet . into_owned () } ; let suggestion = match parens { Parens :: Needed => format ! ("({expr_snippet})") , Parens :: Unneeded => expr_snippet , } ; span_lint_and_sugg (cx , IDENTITY_OP , span , "this operation has no effect" , "consider reducing it to" , suggestion , applicability ,) ; }
};
}
