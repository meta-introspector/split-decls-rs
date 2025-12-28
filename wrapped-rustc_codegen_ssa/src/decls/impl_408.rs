macro_rules! deps {
    () => {
        ExpectedPointerMutability!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl IntoDiagArg for ExpectedPointerMutability { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { match self { ExpectedPointerMutability :: Mut => DiagArgValue :: Str (Cow :: Borrowed ("*mut")) , ExpectedPointerMutability :: Not => DiagArgValue :: Str (Cow :: Borrowed ("*_")) , } } }
    };
}

impl_408!()