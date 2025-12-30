// Generated macro for machreg_to_fpr (function)
macro_rules! Depcrate_isa_s390x_inst_emitmachreg_to_fpr {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"machreg_to_fpr"}
// Dependencies: {}
fn machreg_to_fpr (m : Reg) -> u8 { assert ! (is_fpr (m)) ; u8 :: try_from (m . to_real_reg () . unwrap () . hw_enc ()) . unwrap () }
};
}
