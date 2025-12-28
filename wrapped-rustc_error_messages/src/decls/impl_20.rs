macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: token :: Token { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_to_string (& self)) } }
    };
}

impl_20!()