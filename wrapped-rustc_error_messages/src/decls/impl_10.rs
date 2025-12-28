macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IntoDiagArg for char { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{self:?}"))) } }
    };
}

impl_10!()