// Generated macro for ShiftOpAndAmt (struct)
macro_rules! Depcrate_isa_aarch64_inst_argsShiftOpAndAmt {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"ShiftOpAndAmt"}
// Dependencies: {}
# [doc = " A shift operator with an amount, guaranteed to be within range."] # [derive (Copy , Clone , Debug)] pub struct ShiftOpAndAmt { # [doc = " The shift operator."] op : ShiftOp , # [doc = " The shift operator amount."] shift : ShiftOpShiftImm , }
};
}
