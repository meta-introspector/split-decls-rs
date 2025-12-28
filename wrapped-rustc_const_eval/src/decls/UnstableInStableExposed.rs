macro_rules! UnstableInStableExposed {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unstable_in_stable_exposed)] pub (crate) struct UnstableInStableExposed { pub gate : String , # [primary_span] pub span : Span , # [help (const_eval_is_function_call)] pub is_function_call : bool , # [doc = " Need to duplicate the field so that fluent also provides it as a variable..."] pub is_function_call2 : bool , # [suggestion (const_eval_unstable_sugg , code = "#[rustc_const_unstable(feature = \"...\", issue = \"...\")]\n" , applicability = "has-placeholders")] pub attr_span : Span , }
    };
}

UnstableInStableExposed!();