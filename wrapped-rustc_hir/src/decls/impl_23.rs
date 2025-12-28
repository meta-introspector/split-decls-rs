macro_rules! deps {
    () => {
        MirDialect!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl IntoDiagArg for MirDialect { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirDialect :: Analysis => "analysis" , MirDialect :: Built => "built" , MirDialect :: Runtime => "runtime" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }
    };
}

impl_23!()