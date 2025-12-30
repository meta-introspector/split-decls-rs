// Generated macro for machreg_to_gpr_or_fpr (function)
macro_rules! Depcrate_isa_s390x_inst_emitmachreg_to_gpr_or_fpr {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"machreg_to_gpr_or_fpr"}
// Dependencies: {}
fn machreg_to_gpr_or_fpr (m : Reg) -> u8 { let reg = u8 :: try_from (m . to_real_reg () . unwrap () . hw_enc ()) . unwrap () ; assert ! (reg < 16) ; reg }
};
}
