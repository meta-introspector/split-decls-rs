// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/compiler-builtins/src/probestack.rs
// Error: expected square brackets
// Problematic line: line 55

//
// The ABI here is that the stack frame size is located in `%rax`. Upon
// return we're not supposed to modify `%rsp` or `%rax`.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
#[rustc_std_internal_symbol]
pub unsafe extern "custom" fn __rust_probestack() {
