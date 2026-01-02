// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/mir/locals.rs
// Error: expected square brackets
// Problematic line: line 15

use crate::mir::{FunctionCx, LocalRef};
use crate::traits::BuilderMethods;

pub(super) struct Locals<'tcx, V> {
    values: IndexVec<mir::Local, LocalRef<'tcx, V>>,
}

