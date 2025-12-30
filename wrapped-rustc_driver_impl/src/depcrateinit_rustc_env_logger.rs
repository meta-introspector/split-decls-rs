// Generated macro for init_rustc_env_logger (function)
macro_rules! Depcrateinit_rustc_env_logger {
() => {
// Module: crate
// Provides: {"init_rustc_env_logger"}
// Dependencies: {}
# [doc = " This allows tools to enable rust logging without having to magically match rustc's"] # [doc = " tracing crate version."] pub fn init_rustc_env_logger (early_dcx : & EarlyDiagCtxt) { init_logger (early_dcx , rustc_log :: LoggerConfig :: from_env ("RUSTC_LOG")) ; }
};
}
