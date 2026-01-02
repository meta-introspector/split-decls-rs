// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_monomorphize/src/mono_checks/abi_check.rs
// Error: expected square brackets
// Problematic line: line 13


use crate::errors;

fn uses_vector_registers(mode: &PassMode, repr: &BackendRepr) -> bool {
    match mode {
        PassMode::Ignore | PassMode::Indirect { .. } => false,
        PassMode::Cast { pad_i32: _, cast } => {
