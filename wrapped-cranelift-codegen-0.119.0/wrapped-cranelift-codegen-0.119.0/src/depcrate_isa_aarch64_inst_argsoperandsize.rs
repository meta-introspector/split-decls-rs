// Generated macro for OperandSize (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsOperandSize {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"OperandSize"}
// Dependencies: {}
# [doc = " Type used to communicate the operand size of a machine instruction, as AArch64 has 32- and"] # [doc = " 64-bit variants of many instructions (and integer registers)."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum OperandSize { # [doc = " 32-bit."] Size32 , # [doc = " 64-bit."] Size64 , }
};
}
