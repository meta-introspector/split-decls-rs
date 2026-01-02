// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
// Error: expected square brackets
// Problematic line: line 7

use cranelift_codegen::ir::{InstructionData, Opcode, Value, ValueDef};
use cranelift_frontend::FunctionBuilder;

/// If the given value was produced by the lowering of `Rvalue::Not` return the input and true,
/// otherwise return the given value and false.
pub(crate) fn maybe_unwrap_bool_not(bcx: &mut FunctionBuilder<'_>, arg: Value) -> (Value, bool) {
    if let ValueDef::Result(arg_inst, 0) = bcx.func.dfg.value_def(arg) {
