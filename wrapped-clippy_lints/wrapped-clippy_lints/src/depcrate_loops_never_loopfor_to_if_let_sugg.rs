// Generated macro for for_to_if_let_sugg (function)
macro_rules! Depcrate_loops_never_loopfor_to_if_let_sugg {
() => {
// Module: crate::loops::never_loop
// Provides: {"for_to_if_let_sugg"}
// Dependencies: {}
fn for_to_if_let_sugg (cx : & LateContext < '_ > , iterator : & Expr < '_ > , pat : & Pat < '_ >) -> String { let pat_snippet = snippet (cx , pat . span , "_") ; let iter_snippet = make_iterator_snippet (cx , iterator , & mut Applicability :: Unspecified) ; format ! ("if let Some({pat_snippet}) = {iter_snippet}.next()") }
};
}
