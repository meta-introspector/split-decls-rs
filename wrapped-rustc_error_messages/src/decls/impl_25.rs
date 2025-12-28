macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl IntoDiagArg for Backtrace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }
    };
}

impl_25!()