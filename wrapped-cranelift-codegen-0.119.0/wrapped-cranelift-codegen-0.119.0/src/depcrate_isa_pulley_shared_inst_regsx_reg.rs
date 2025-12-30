// Generated macro for x_reg (function)
macro_rules! Depcrate_isa_pulley_shared_inst_regsx_reg {
() => {
// Module: crate::isa::pulley_shared::inst::regs
// Provides: {"x_reg"}
// Dependencies: {}
# [inline] pub fn x_reg (enc : usize) -> Reg { let p = PReg :: new (enc , RegClass :: Int) ; let v = VReg :: new (p . index () , p . class ()) ; Reg :: from (v) }
};
}
