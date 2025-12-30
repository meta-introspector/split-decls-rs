// Generated macro for get_vecreg_for_arg (function)
macro_rules! Depcrate_isa_s390x_abiget_vecreg_for_arg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_vecreg_for_arg"}
// Dependencies: {}
fn get_vecreg_for_arg (idx : usize) -> Option < Reg > { match idx { 0 => Some (regs :: vr (24)) , 1 => Some (regs :: vr (25)) , 2 => Some (regs :: vr (26)) , 3 => Some (regs :: vr (27)) , 4 => Some (regs :: vr (28)) , 5 => Some (regs :: vr (29)) , 6 => Some (regs :: vr (30)) , 7 => Some (regs :: vr (31)) , _ => None , } }
};
}
