macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl IntoDiagArg for rustc_span :: Symbol { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_ident_string () . into_diag_arg (path) } }
    };
}

impl_12!()