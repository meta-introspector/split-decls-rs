macro_rules! deps {
    () => {
        IntoDiagArg!();
        DiagArgValue!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl IntoDiagArg for ast :: util :: parser :: ExprPrecedence { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Number (self as i32) } }
    };
}

impl_26!()