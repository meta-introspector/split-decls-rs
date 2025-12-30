// Generated macro for cargo_bin (function)
macro_rules! Depcrate_cargocargo_bin {
() => {
// Module: crate::cargo
// Provides: {"cargo_bin"}
// Dependencies: {}
fn cargo_bin () -> & 'static ffi :: OsStr { static CARGO_BIN : std :: sync :: OnceLock < ffi :: OsString > = std :: sync :: OnceLock :: new () ; CARGO_BIN . get_or_init (| | env :: var_os ("CARGO") . unwrap_or_else (| | "cargo" . into ())) }
};
}
