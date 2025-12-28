macro_rules! NonConstIntrinsic {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_intrinsic)] pub (crate) struct NonConstIntrinsic { # [primary_span] pub span : Span , pub name : Symbol , pub kind : ConstContext , }
    };
}

NonConstIntrinsic!();