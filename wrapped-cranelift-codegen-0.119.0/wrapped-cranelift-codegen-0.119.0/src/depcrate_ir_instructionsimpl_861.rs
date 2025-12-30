// Generated macro for impl_861 (impl)
macro_rules! Depcrate_ir_instructionsimpl_861 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_861"}
// Dependencies: {}
impl Opcode { # [doc = " Get the instruction format for this opcode."] pub fn format (self) -> InstructionFormat { OPCODE_FORMAT [self as usize - 1] } # [doc = " Get the constraint descriptor for this opcode."] # [doc = " Panic if this is called on `NotAnOpcode`."] pub fn constraints (self) -> OpcodeConstraints { OPCODE_CONSTRAINTS [self as usize - 1] } # [doc = " Is this instruction a GC safepoint?"] # [doc = ""] # [doc = " Safepoints are all kinds of calls, except for tail calls."] # [inline] pub fn is_safepoint (self) -> bool { self . is_call () && ! self . is_return () } }
};
}
