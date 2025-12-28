macro_rules! deps {
    () => {
        Res!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < Id > IntoDiagArg for Res < Id > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . descr ())) } }
    };
}

impl_61!()