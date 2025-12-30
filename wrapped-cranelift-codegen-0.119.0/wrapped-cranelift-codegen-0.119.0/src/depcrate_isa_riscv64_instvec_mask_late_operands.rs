// Generated macro for vec_mask_late_operands (function)
macro_rules! Depcrate_isa_riscv64_instvec_mask_late_operands {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"vec_mask_late_operands"}
// Dependencies: {}
fn vec_mask_late_operands (mask : & mut VecOpMasking , collector : & mut impl OperandVisitor) { match mask { VecOpMasking :: Enabled { reg } => { collector . reg_fixed_late_use (reg , pv_reg (0) . into ()) ; } VecOpMasking :: Disabled => { } } }
};
}
