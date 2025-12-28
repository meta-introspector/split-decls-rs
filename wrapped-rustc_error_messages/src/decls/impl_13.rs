macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a > IntoDiagArg for & 'a str { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } }
    };
}

impl_13!()