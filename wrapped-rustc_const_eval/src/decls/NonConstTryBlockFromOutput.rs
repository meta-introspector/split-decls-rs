macro_rules! NonConstTryBlockFromOutput {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_try_block_from_output , code = E0015)] pub struct NonConstTryBlockFromOutput < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstTryBlockFromOutput!()