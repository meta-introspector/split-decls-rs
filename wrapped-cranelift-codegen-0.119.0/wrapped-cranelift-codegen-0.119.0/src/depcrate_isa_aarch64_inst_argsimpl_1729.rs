// Generated macro for impl_1729 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1729 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1729"}
// Dependencies: {}
impl ShiftOpAndAmt { # [doc = " Create a new shift operator with an amount."] pub fn new (op : ShiftOp , shift : ShiftOpShiftImm) -> ShiftOpAndAmt { ShiftOpAndAmt { op , shift } } # [doc = " Get the shift op."] pub fn op (& self) -> ShiftOp { self . op } # [doc = " Get the shift amount."] pub fn amt (& self) -> ShiftOpShiftImm { self . shift } }
};
}
