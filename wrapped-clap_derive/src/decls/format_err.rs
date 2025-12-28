macro_rules! format_err {
    () => {
        macro_rules ! format_err { ($ obj : expr , $ ($ format : tt) +) => { { # [allow (unused_imports)] use $ crate :: utils :: error ::*; let msg = format ! ($ ($ format) +) ; $ obj . EXPECTED_Span_OR_ToTokens (msg) } } ; }
    };
}

format_err!()