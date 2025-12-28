macro_rules! deps {
    () => {
        StabilityOutsideStd!();
    };
}

macro_rules! reject_outside_std {
    () => {
        deps!();
        macro_rules ! reject_outside_std { ($ cx : ident) => { if !$ cx . features () . staged_api () { $ cx . emit_err (session_diagnostics :: StabilityOutsideStd { span : $ cx . attr_span }) ; return ; } } ; }
    };
}

reject_outside_std!()