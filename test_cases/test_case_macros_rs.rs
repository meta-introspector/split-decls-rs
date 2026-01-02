// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/macros.rs
// Error: expected square brackets
// Problematic line: line 5


// Helper macro used to trigger const eval errors when the const generic immediate value `imm` is
// not a round number.
#[allow(unused)]
macro_rules! static_assert_rounding {
    ($imm:ident) => {
        static_assert!(
