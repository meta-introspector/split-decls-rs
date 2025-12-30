// Generated macro for get_fltreg_for_ret (function)
macro_rules! Depcrate_isa_s390x_abiget_fltreg_for_ret {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_fltreg_for_ret"}
// Dependencies: {}
fn get_fltreg_for_ret (idx : usize) -> Option < Reg > { match idx { 0 => Some (regs :: vr (0)) , 1 => Some (regs :: vr (2)) , 2 => Some (regs :: vr (4)) , 3 => Some (regs :: vr (6)) , _ => None , } }
};
}
