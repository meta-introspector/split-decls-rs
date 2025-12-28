macro_rules! deps {
    () => {
        RustcVersion!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl IntoDiagArg for RustcVersion { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string ())) } }
    };
}

impl_480!()