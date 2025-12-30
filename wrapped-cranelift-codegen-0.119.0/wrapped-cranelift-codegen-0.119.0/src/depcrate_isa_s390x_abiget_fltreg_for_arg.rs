// Generated macro for get_fltreg_for_arg (function)
macro_rules! Depcrate_isa_s390x_abiget_fltreg_for_arg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_fltreg_for_arg"}
// Dependencies: {}
fn get_fltreg_for_arg (idx : usize) -> Option < Reg > { match idx { 0 => Some (regs :: vr (0)) , 1 => Some (regs :: vr (2)) , 2 => Some (regs :: vr (4)) , 3 => Some (regs :: vr (6)) , _ => None , } }
};
}
