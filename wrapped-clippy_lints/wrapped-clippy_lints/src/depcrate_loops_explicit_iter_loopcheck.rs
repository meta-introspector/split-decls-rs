// Generated macro for check (function)
macro_rules! Depcrate_loops_explicit_iter_loopcheck {
() => {
// Module: crate::loops::explicit_iter_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , self_arg : & Expr < '_ > , call_expr : & Expr < '_ > , msrv : Msrv , enforce_iter_loop_reborrow : bool ,) { let Some ((adjust , ty)) = is_ref_iterable (cx , self_arg , call_expr , enforce_iter_loop_reborrow , msrv) else { return ; } ; if let ty :: Array (_ , count) = * ty . peel_refs () . kind () { if ! ty . is_ref () { if ! msrv . meets (cx , msrvs :: ARRAY_INTO_ITERATOR) { return ; } } else if count . try_to_target_usize (cx . tcx) . is_none_or (| x | x > 32) && ! msrv . meets (cx , msrvs :: ARRAY_IMPL_ANY_LEN) { return ; } } let mut applicability = Applicability :: MachineApplicable ; let object = snippet_with_context (cx , self_arg . span , call_expr . span . ctxt () , "_" , & mut applicability) . 0 ; span_lint_and_sugg (cx , EXPLICIT_ITER_LOOP , call_expr . span , "it is more concise to loop over references to containers instead of using explicit \
         iteration methods" , "to write this more concisely, try" , format ! ("{}{object}" , adjust . display ()) , applicability ,) ; }
};
}
