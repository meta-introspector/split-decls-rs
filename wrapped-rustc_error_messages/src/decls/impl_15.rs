macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'a > IntoDiagArg for Cow < 'a , str > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . into_owned ())) } }
    };
}

impl_15!();