// Generated macro for CondBrKind (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsCondBrKind {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"CondBrKind"}
// Dependencies: {}
# [doc = " The kind of conditional branch: the common-case-optimized \"reg-is-zero\" /"] # [doc = " \"reg-is-nonzero\" variants, or the generic one that tests the machine"] # [doc = " condition codes."] # [derive (Clone , Copy , Debug)] pub enum CondBrKind { # [doc = " Condition: given register is zero."] Zero (Reg , OperandSize) , # [doc = " Condition: given register is nonzero."] NotZero (Reg , OperandSize) , # [doc = " Condition: the given condition-code test is true."] Cond (Cond) , }
};
}
