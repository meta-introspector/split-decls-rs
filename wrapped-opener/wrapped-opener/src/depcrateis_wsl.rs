// Generated macro for is_wsl (function)
macro_rules! Depcrateis_wsl {
() => {
// Module: crate
// Provides: {"is_wsl"}
// Dependencies: {}
# [cfg (not (target_os = "linux"))] fn is_wsl () -> bool { false }
};
}
