// Generated macro for machreg_to_gpr (function)
macro_rules! Depcrate_isa_s390x_inst_emitmachreg_to_gpr {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"machreg_to_gpr"}
// Dependencies: {}
fn machreg_to_gpr (m : Reg) -> u8 { assert_eq ! (m . class () , RegClass :: Int) ; u8 :: try_from (m . to_real_reg () . unwrap () . hw_enc ()) . unwrap () }
};
}
