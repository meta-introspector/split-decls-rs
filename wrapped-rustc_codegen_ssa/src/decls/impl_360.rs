macro_rules! deps {
    () => {
        DebugArgPath!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl IntoDiagArg for DebugArgPath < '_ > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{:?}" , self . 0))) } }
    };
}

impl_360!();