macro_rules! IcePath {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_ice_path)] pub (crate) struct IcePath { pub path : std :: path :: PathBuf , }
    };
}

IcePath!();