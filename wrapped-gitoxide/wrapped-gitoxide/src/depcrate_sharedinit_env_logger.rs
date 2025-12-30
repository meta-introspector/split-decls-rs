// Generated macro for init_env_logger (function)
macro_rules! Depcrate_sharedinit_env_logger {
() => {
// Module: crate::shared
// Provides: {"init_env_logger"}
// Dependencies: {}
# [doc = " If verbose is true, the env logger will be forcibly set to 'info' logging level. Otherwise env logging facilities"] # [doc = " will just be initialized."] # [allow (unused)] pub fn init_env_logger () { if cfg ! (feature = "small") { env_logger :: Builder :: from_env (env_logger :: Env :: default () . default_filter_or ("info")) . format_module_path (false) . init () ; } else { env_logger :: init () ; } }
};
}
