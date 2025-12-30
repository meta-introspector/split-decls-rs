// Generated macro for ProgramPoint (enum)
macro_rules! Depcrate_ir_progpointProgramPoint {
() => {
// Module: crate::ir::progpoint
// Provides: {"ProgramPoint"}
// Dependencies: {}
# [doc = " A `ProgramPoint` represents a position in a function where the live range of an SSA value can"] # [doc = " begin or end. It can be either:"] # [doc = ""] # [doc = " 1. An instruction or"] # [doc = " 2. A block header."] # [doc = ""] # [doc = " This corresponds more or less to the lines in the textual form of Cranelift IR."] # [derive (PartialEq , Eq , Clone , Copy)] pub enum ProgramPoint { # [doc = " An instruction in the function."] Inst (Inst) , # [doc = " A block header."] Block (Block) , }
};
}
