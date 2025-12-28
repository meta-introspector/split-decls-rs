macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl IntoDiagArg for Vec < char > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: StrListSepByAnd (self . into_iter () . map (| c | Cow :: Owned (format ! ("{c:?}"))) . collect () ,) } }
    };
}

impl_11!()