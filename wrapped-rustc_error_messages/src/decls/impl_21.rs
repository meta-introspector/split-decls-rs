macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: token :: TokenKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_kind_to_string (& self)) } }
    };
}

impl_21!()