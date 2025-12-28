macro_rules! NonConstForLoopIntoIter {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_for_loop_into_iter , code = E0015)] pub struct NonConstForLoopIntoIter < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstForLoopIntoIter!()