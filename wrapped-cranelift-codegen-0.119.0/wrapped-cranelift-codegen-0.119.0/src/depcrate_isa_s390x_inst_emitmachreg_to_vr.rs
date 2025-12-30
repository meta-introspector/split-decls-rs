// Generated macro for machreg_to_vr (function)
macro_rules! Depcrate_isa_s390x_inst_emitmachreg_to_vr {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"machreg_to_vr"}
// Dependencies: {}
fn machreg_to_vr (m : Reg) -> u8 { assert_eq ! (m . class () , RegClass :: Float) ; u8 :: try_from (m . to_real_reg () . unwrap () . hw_enc ()) . unwrap () }
};
}
