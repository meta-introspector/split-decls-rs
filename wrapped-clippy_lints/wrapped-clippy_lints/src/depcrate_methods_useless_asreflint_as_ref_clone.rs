// Generated macro for lint_as_ref_clone (function)
macro_rules! Depcrate_methods_useless_asreflint_as_ref_clone {
() => {
// Module: crate::methods::useless_asref
// Provides: {"lint_as_ref_clone"}
// Dependencies: {}
fn lint_as_ref_clone (cx : & LateContext < '_ > , span : Span , recvr : & hir :: Expr < '_ > , call_name : Symbol) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , USELESS_ASREF , span , format ! ("this call to `{call_name}.map(...)` does nothing") , "try" , format ! ("{}.clone()" , snippet_with_applicability (cx , recvr . span , ".." , & mut applicability)) , applicability ,) ; }
};
}
