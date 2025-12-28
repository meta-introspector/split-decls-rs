macro_rules! deps {
    () => {
        RegionName!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl rustc_errors :: IntoDiagArg for RegionName { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { self . to_string () . into_diag_arg (path) } }
    };
}

impl_85!();