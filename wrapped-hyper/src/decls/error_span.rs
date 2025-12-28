macro_rules! error_span {
    () => {
        macro_rules ! error_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: error_span ! ($ ($ arg) +) ; _span . entered () } } } }
    };
}

error_span!()