macro_rules! deps {
    () => {
        DiagArgFromDisplay!();
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl IntoDiagArg for DiagArgFromDisplay < '_ > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . 0 . to_string () . into_diag_arg (path) } }
    };
}

impl_1!()