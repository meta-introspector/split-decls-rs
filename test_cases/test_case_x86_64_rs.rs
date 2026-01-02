// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/compiler-builtins/src/x86_64.rs
// Error: expected square brackets
// Problematic line: line 10


// NOTE These functions are never mangled as they are not tested against compiler-rt

intrinsics! {
    #[unsafe(naked)]
    #[cfg(any(all(windows, target_env = "gnu"), target_os = "cygwin", target_os = "uefi"))]
    pub unsafe extern "custom" fn ___chkstk_ms() {
