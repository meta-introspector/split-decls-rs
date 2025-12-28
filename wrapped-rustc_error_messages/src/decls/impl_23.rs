macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl IntoDiagArg for rustc_data_structures :: small_c_str :: SmallCStr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
    };
}

impl_23!()