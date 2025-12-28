macro_rules! deps {
    () => {
        AttrPath!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl IntoDiagArg for AttrPath { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } }
    };
}

impl_157!()