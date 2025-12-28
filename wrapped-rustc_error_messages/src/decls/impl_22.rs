macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl IntoDiagArg for std :: ffi :: CString { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
    };
}

impl_22!()