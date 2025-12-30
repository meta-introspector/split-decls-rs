// Generated macro for ShiftOp (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsShiftOp {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"ShiftOp"}
// Dependencies: {}
# [doc = " A shift operator for a register or immediate."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum ShiftOp { # [doc = " Logical shift left."] LSL = 0b00 , # [doc = " Logical shift right."] LSR = 0b01 , # [doc = " Arithmetic shift right."] ASR = 0b10 , # [doc = " Rotate right."] ROR = 0b11 , }
};
}
