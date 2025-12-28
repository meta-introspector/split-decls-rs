macro_rules! init_rustc_env_logger {
    () => {
        # [doc = " This allows tools to enable rust logging without having to magically match rustc's"] # [doc = " tracing crate version."] pub fn init_rustc_env_logger (early_dcx : & EarlyDiagCtxt) { init_logger (early_dcx , rustc_log :: LoggerConfig :: from_env ("RUSTC_LOG")) ; }
    };
}

init_rustc_env_logger!()