// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/compiler-builtins/src/float/cmp.rs
// Error: expected square brackets
// Problematic line: line 9


// Taken from LLVM config:
// https://github.com/llvm/llvm-project/blob/0cf3c437c18ed27d9663d87804a9a15ff6874af2/compiler-rt/lib/builtins/fp_compare_impl.inc#L11-L27
cfg_if! {
    if #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))] {
        // Aarch64 uses `int` rather than a pointer-sized value.
        pub type CmpResult = i32;
