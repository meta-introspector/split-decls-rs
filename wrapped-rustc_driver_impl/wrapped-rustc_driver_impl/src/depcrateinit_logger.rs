// Generated macro for init_logger (function)
macro_rules! Depcrateinit_logger {
() => {
// Module: crate
// Provides: {"init_logger"}
// Dependencies: {}
# [doc = " This allows tools to enable rust logging without having to magically match rustc's"] # [doc = " tracing crate version. In contrast to `init_rustc_env_logger` it allows you to choose"] # [doc = " the logger config directly rather than having to set an environment variable."] pub fn init_logger (early_dcx : & EarlyDiagCtxt , cfg : rustc_log :: LoggerConfig) { if let Err (error) = rustc_log :: init_logger (cfg) { early_dcx . early_fatal (error . to_string ()) ; } }
};
}
