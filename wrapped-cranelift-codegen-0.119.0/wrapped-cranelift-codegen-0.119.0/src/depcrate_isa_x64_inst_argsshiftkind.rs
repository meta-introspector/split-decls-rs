// Generated macro for ShiftKind (enum)
macro_rules! Depcrate_isa_x64_inst_argsShiftKind {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"ShiftKind"}
// Dependencies: {}
# [doc = " These indicate the form of a scalar shift/rotate: left, signed right, unsigned right."] # [derive (Clone , Copy)] pub enum ShiftKind { # [doc = " Left shift."] ShiftLeft , # [doc = " Inserts zeros in the most significant bits."] ShiftRightLogical , # [doc = " Replicates the sign bit in the most significant bits."] ShiftRightArithmetic , # [doc = " Left rotation."] RotateLeft , # [doc = " Right rotation."] RotateRight , }
};
}
