// Generated macro for is_fpr (function)
macro_rules! Depcrate_isa_s390x_inst_regsis_fpr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"is_fpr"}
// Dependencies: {}
# [doc = " Test whether a vector register is overlapping an FPR."] pub fn is_fpr (r : Reg) -> bool { let r = r . to_real_reg () . unwrap () ; assert ! (r . class () == RegClass :: Float) ; return r . hw_enc () < 16 ; }
};
}
