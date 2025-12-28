macro_rules! deps {
    () => {
        CguReuse!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl IntoDiagArg for CguReuse { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string ())) } }
    };
}

impl_5!();