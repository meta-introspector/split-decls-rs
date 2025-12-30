// Generated macro for rust_psm_on_stack (function)
macro_rules! Depcraterust_psm_on_stack {
() => {
// Module: crate
// Provides: {"rust_psm_on_stack"}
// Dependencies: {}
# [cfg (all (switchable_stack , not (target_os = "windows")))] # [inline (always)] unsafe fn rust_psm_on_stack (data : usize , return_ptr : usize , callback : extern_item ! (unsafe fn (usize , usize)) , sp : * mut u8 , _ : * mut u8 ,) { _rust_psm_on_stack (data , return_ptr , callback , sp) }
};
}
