// Generated macro for f_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_regsf_reg {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"f_reg"}
// Dependencies: {}
# [inline] pub fn f_reg (enc : usize) -> Reg { let p_reg = PReg :: new (enc , RegClass :: Float) ; let v_reg = VReg :: new (p_reg . index () , p_reg . class ()) ; Reg :: from (v_reg) }
};
}
