macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl IntoDiagArg for bool { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { if self { DiagArgValue :: Str (Cow :: Borrowed ("true")) } else { DiagArgValue :: Str (Cow :: Borrowed ("false")) } } }
    };
}

impl_9!();