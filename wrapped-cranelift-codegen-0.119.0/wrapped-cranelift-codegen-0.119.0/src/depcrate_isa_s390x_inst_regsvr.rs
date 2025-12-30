// Generated macro for vr (function)
macro_rules! Depcrate_isa_s390x_inst_regsvr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"vr"}
// Dependencies: {}
# [doc = " Get a reference to a VR (vector register)."] pub fn vr (num : u8) -> Reg { Reg :: from (vr_preg (num)) }
};
}
