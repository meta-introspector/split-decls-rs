// Generated macro for x_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_regsx_reg {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"x_reg"}
// Dependencies: {}
# [inline] pub fn x_reg (enc : usize) -> Reg { let p_reg = PReg :: new (enc , RegClass :: Int) ; let v_reg = VReg :: new (p_reg . index () , p_reg . class ()) ; Reg :: from (v_reg) }
};
}
