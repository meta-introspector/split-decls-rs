// Generated macro for enable_virtual_terminal_processing (function)
macro_rules! Depcrate_windowsenable_virtual_terminal_processing {
() => {
// Module: crate::windows
// Provides: {"enable_virtual_terminal_processing"}
// Dependencies: {}
# [doc = " Raw `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on stdout/stderr"] # [cfg (windows)] pub fn enable_virtual_terminal_processing () -> std :: io :: Result < () > { windows_console :: enable_virtual_terminal_processing () }
};
}
