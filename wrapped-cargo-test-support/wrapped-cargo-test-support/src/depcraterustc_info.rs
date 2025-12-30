// Generated macro for rustc_info (function)
macro_rules! Depcraterustc_info {
() => {
// Module: crate
// Provides: {"rustc_info"}
// Dependencies: {}
fn rustc_info () -> & 'static RustcInfo { static RUSTC_INFO : OnceLock < RustcInfo > = OnceLock :: new () ; RUSTC_INFO . get_or_init (RustcInfo :: new) }
};
}
