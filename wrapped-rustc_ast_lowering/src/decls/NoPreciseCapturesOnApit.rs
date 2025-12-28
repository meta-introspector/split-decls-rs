macro_rules! NoPreciseCapturesOnApit {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_no_precise_captures_on_apit)] pub (crate) struct NoPreciseCapturesOnApit { # [primary_span] pub span : Span , }
    };
}

NoPreciseCapturesOnApit!()