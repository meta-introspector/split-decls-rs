macro_rules! deps {
    () => {
        ReturnLikeStatementKind!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl IntoDiagArg for ReturnLikeStatementKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let kind = match self { Self :: Return => "return" , Self :: Become => "become" , } . into () ; DiagArgValue :: Str (kind) } }
    };
}

impl_62!();