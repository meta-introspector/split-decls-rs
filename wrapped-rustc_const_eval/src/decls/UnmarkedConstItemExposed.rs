macro_rules! UnmarkedConstItemExposed {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unmarked_const_item_exposed)] # [help] pub (crate) struct UnmarkedConstItemExposed { # [primary_span] pub span : Span , pub def_path : String , }
    };
}

UnmarkedConstItemExposed!()