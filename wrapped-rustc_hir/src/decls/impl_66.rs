macro_rules! deps {
    () => {
        Namespace!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl IntoDiagArg for Namespace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . descr ())) } }
    };
}

impl_66!();