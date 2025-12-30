// Generated macro for is_cargo_env_var (function)
macro_rules! Depcrate_envis_cargo_env_var {
() => {
// Module: crate::env
// Provides: {"is_cargo_env_var"}
// Dependencies: {}
# [doc = " Returns `true` if an environment variable from `env!` is one used by Cargo."] fn is_cargo_env_var (var : & str) -> bool { var . starts_with ("CARGO_") || var . starts_with ("DEP_") || matches ! (var , "OUT_DIR" | "OPT_LEVEL" | "PROFILE" | "HOST" | "TARGET") }
};
}
