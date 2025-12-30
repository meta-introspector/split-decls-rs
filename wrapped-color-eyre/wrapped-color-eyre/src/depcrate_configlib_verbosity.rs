// Generated macro for lib_verbosity (function)
macro_rules! Depcrate_configlib_verbosity {
() => {
// Module: crate::config
// Provides: {"lib_verbosity"}
// Dependencies: {}
pub (crate) fn lib_verbosity () -> Verbosity { match env :: var ("RUST_LIB_BACKTRACE") . or_else (| _ | env :: var ("RUST_BACKTRACE")) { Ok (s) if s == "full" => Verbosity :: Full , Ok (s) if s != "0" => Verbosity :: Medium , _ => Verbosity :: Minimal , } }
};
}
