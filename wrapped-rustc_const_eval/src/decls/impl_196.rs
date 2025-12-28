macro_rules! deps {
    () => {
        InternKind!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl rustc_errors :: IntoDiagArg for InternKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { InternKind :: Static (Mutability :: Not) => "static" , InternKind :: Static (Mutability :: Mut) => "static_mut" , InternKind :: Constant => "const" , InternKind :: Promoted => "promoted" , })) } }
    };
}

impl_196!()