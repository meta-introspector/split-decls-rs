macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl IntoDiagArg for DiagArgValue { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self } }
    };
}

impl_34!()