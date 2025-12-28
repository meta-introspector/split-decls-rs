macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: Expr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: expr_to_string (& self))) } }
    };
}

impl_18!()