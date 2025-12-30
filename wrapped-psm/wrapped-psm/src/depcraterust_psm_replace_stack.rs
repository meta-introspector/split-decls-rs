// Generated macro for rust_psm_replace_stack (function)
macro_rules! Depcraterust_psm_replace_stack {
() => {
// Module: crate
// Provides: {"rust_psm_replace_stack"}
// Dependencies: {}
# [cfg (all (switchable_stack , not (target_os = "windows")))] # [inline (always)] unsafe fn rust_psm_replace_stack (data : usize , callback : extern_item ! (unsafe fn (usize) -> !) , sp : * mut u8 , _ : * mut u8 ,) -> ! { _rust_psm_replace_stack (data , callback , sp) }
};
}
