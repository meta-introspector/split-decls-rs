macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }
    };
}

impl_54!()