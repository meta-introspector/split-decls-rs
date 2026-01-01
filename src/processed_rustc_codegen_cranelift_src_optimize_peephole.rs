// SRC: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=4 */
// Peephole optimizations that can be performed while creating clif ir.

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{InstructionData, Opcode, Value, ValueDef};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=20 */
use cranelift_frontend::FunctionBuilder;

/// If the given value was produced by the lowering of `Rvalue::Not` return the input and true,
/// otherwise return the given value and false.
pub(crate) fn maybe_unwrap_bool_not(bcx: &mut FunctionBuilder<'_>, arg: Value) -> (Value, bool) {
    if let ValueDef::Result(arg_inst, 0) = bcx.func.dfg.value_def(arg) {
        match bcx.func.dfg.insts[arg_inst] {
            // This is the lowering of `Rvalue::Not`
            InstructionData::IntCompareImm {
                opcode: Opcode::IcmpImm,
                cond: IntCC::Equal,
                arg,
                imm,
            } if imm.bits() == 0 => (arg, true),
            _ => (arg, false),
        }
    } else {
        (arg, false)
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=24 */

/// Returns whether the branch is statically known to be taken or `None` if it isn't statically known.
pub(crate) fn maybe_known_branch_taken(
    bcx: &FunctionBuilder<'_>,
    arg: Value,
    test_zero: bool,
) -> Option<bool> {
    let arg_inst = if let ValueDef::Result(arg_inst, 0) = bcx.func.dfg.value_def(arg) {
        arg_inst
    } else {
        return None;
    };

    match bcx.func.dfg.insts[arg_inst] {
        InstructionData::UnaryImm { opcode: Opcode::Iconst, imm } => {
            if test_zero {
                Some(imm.bits() == 0)
            } else {
                Some(imm.bits() != 0)
            }
        }
        _ => None,
    }
}