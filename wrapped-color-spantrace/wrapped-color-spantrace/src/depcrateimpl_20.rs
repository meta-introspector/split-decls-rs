// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Verbosity { fn lib_from_env () -> Self { Self :: convert_env (env :: var ("RUST_LIB_BACKTRACE") . or_else (| _ | env :: var ("RUST_BACKTRACE")) . ok () ,) } fn convert_env (env : Option < String >) -> Self { match env { Some (ref x) if x == "full" => Verbosity :: Full , Some (_) => Verbosity :: Medium , None => Verbosity :: Minimal , } } }
};
}
