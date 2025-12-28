macro_rules! deps {
    () => {
        ConstContext!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl IntoDiagArg for ConstContext { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { ConstContext :: ConstFn => "const_fn" , ConstContext :: Static (_) => "static" , ConstContext :: Const { .. } => "const" , })) } }
    };
}

impl_216!();