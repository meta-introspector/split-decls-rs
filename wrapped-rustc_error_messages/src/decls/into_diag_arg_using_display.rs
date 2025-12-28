macro_rules! deps {
    () => {
        DiagArgValue!();
        IntoDiagArg!();
    };
}

macro_rules! into_diag_arg_using_display {
    () => {
        deps!();
        # [macro_export] macro_rules ! into_diag_arg_using_display { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { self . to_string () . into_diag_arg (path) } }) + } }
    };
}

into_diag_arg_using_display!()