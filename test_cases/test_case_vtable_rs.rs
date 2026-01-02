// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/vtable.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::constant::data_id_for_vtable;
use crate::prelude::*;

pub(crate) fn vtable_memflags() -> MemFlags {
    let mut flags = MemFlags::trusted(); // A vtable access is always aligned and will never trap.
    flags.set_readonly(); // A vtable is always read-only.
    flags
