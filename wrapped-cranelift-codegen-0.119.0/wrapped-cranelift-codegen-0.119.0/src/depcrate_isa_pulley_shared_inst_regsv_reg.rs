// Generated macro for v_reg (function)
macro_rules! Depcrate_isa_pulley_shared_inst_regsv_reg {
() => {
// Module: crate::isa::pulley_shared::inst::regs
// Provides: {"v_reg"}
// Dependencies: {}
# [inline] pub fn v_reg (enc : usize) -> Reg { let p = PReg :: new (enc , RegClass :: Vector) ; let v = VReg :: new (p . index () , p . class ()) ; Reg :: from (v) }
};
}
