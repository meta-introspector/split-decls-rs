macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl IntoDiagArg for PathBuf { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }
    };
}

impl_17!()