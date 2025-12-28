macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: path_to_string (& self))) } }
    };
}

impl_19!()