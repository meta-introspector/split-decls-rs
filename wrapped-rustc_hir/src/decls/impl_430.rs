macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl IntoDiagArg for Limit { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (& mut None) } }
    };
}

impl_430!()