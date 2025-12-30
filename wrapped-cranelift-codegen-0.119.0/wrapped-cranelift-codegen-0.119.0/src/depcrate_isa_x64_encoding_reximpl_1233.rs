// Generated macro for impl_1233 (impl)
macro_rules! Depcrate_isa_x64_encoding_reximpl_1233 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"impl_1233"}
// Dependencies: {}
# [doc = " Generate Rex flags for an OperandSize/register tuple."] impl From < (OperandSize , Reg) > for RexFlags { fn from ((size , reg) : (OperandSize , Reg)) -> Self { let mut rex = RexFlags :: from (size) ; if size == OperandSize :: Size8 { rex . always_emit_if_8bit_needed (reg) ; } rex } }
};
}
