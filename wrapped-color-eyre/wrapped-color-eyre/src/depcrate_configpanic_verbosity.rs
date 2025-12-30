// Generated macro for panic_verbosity (function)
macro_rules! Depcrate_configpanic_verbosity {
() => {
// Module: crate::config
// Provides: {"panic_verbosity"}
// Dependencies: {}
pub (crate) fn panic_verbosity () -> Verbosity { match env :: var ("RUST_BACKTRACE") { Ok (s) if s == "full" => Verbosity :: Full , Ok (s) if s != "0" => Verbosity :: Medium , _ => Verbosity :: Minimal , } }
};
}
