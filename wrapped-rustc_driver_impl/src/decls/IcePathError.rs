macro_rules! deps {
    () => {
        IcePathErrorEnv!();
    };
}

macro_rules! IcePathError {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (driver_impl_ice_path_error)] pub (crate) struct IcePathError { pub path : std :: path :: PathBuf , pub error : String , # [subdiagnostic] pub env_var : Option < IcePathErrorEnv > , }
    };
}

IcePathError!()