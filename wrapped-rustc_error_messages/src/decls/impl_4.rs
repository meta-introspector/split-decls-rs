macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a , T : Clone + IntoDiagArg > IntoDiagArg for & 'a T { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . clone () . into_diag_arg (path) } }
    };
}

impl_4!()