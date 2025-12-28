macro_rules! YieldInClosure {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_yield_in_closure)] pub (crate) struct YieldInClosure { # [primary_span] pub span : Span , # [suggestion (code = "#[coroutine] " , applicability = "maybe-incorrect" , style = "verbose")] pub suggestion : Option < Span > , }
    };
}

YieldInClosure!()