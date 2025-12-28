macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: FloatTy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . name_str ())) } }
    };
}

impl_27!()