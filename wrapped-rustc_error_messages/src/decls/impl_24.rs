macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: Visibility { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let s = pprust :: vis_to_string (& self) ; let s = s . trim_end () . to_string () ; DiagArgValue :: Str (Cow :: Owned (s)) } }
    };
}

impl_24!()