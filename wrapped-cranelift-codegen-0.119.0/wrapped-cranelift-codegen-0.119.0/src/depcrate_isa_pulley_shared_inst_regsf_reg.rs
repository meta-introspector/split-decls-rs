// Generated macro for f_reg (function)
macro_rules! Depcrate_isa_pulley_shared_inst_regsf_reg {
() => {
// Module: crate::isa::pulley_shared::inst::regs
// Provides: {"f_reg"}
// Dependencies: {}
# [inline] pub fn f_reg (enc : usize) -> Reg { let p = PReg :: new (enc , RegClass :: Float) ; let v = VReg :: new (p . index () , p . class ()) ; Reg :: from (v) }
};
}
