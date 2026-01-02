// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/dwarf_const.rs
// Error: expected square brackets
// Problematic line: line 5


use libc::c_uint;

/// Helper macro to let us redeclare gimli's constants as our own constants
/// with a different type, with less risk of copy-paste errors.
macro_rules! declare_constant {
    (
