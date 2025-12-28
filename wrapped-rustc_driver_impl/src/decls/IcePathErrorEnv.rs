macro_rules! IcePathErrorEnv {
    () => {
        # [derive (Subdiagnostic)] # [note (driver_impl_ice_path_error_env)] pub (crate) struct IcePathErrorEnv { pub env_var : std :: path :: PathBuf , }
    };
}

IcePathErrorEnv!();