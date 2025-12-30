// Generated macro for impl_1232 (impl)
macro_rules! Depcrate_isa_x64_encoding_reximpl_1232 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"impl_1232"}
// Dependencies: {}
# [doc = " Generate the proper Rex flags for the given operand size."] impl From < OperandSize > for RexFlags { fn from (size : OperandSize) -> Self { match size { OperandSize :: Size64 => RexFlags :: set_w () , _ => RexFlags :: clear_w () , } } }
};
}
