macro_rules! span {
    () => {
        macro_rules ! span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: span ! ($ ($ arg) +) ; _span . entered () } } } }
    };
}

span!()