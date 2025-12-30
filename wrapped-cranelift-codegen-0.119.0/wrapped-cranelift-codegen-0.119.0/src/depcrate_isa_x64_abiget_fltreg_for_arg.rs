// Generated macro for get_fltreg_for_arg (function)
macro_rules! Depcrate_isa_x64_abiget_fltreg_for_arg {
() => {
// Module: crate::isa::x64::abi
// Provides: {"get_fltreg_for_arg"}
// Dependencies: {}
fn get_fltreg_for_arg (call_conv : CallConv , idx : usize , arg_idx : usize) -> Option < Reg > { let is_fastcall = call_conv == CallConv :: WindowsFastcall ; let i = if is_fastcall { arg_idx } else { idx } ; match (i , is_fastcall) { (0 , false) => Some (regs :: xmm0 ()) , (1 , false) => Some (regs :: xmm1 ()) , (2 , false) => Some (regs :: xmm2 ()) , (3 , false) => Some (regs :: xmm3 ()) , (4 , false) => Some (regs :: xmm4 ()) , (5 , false) => Some (regs :: xmm5 ()) , (6 , false) => Some (regs :: xmm6 ()) , (7 , false) => Some (regs :: xmm7 ()) , (0 , true) => Some (regs :: xmm0 ()) , (1 , true) => Some (regs :: xmm1 ()) , (2 , true) => Some (regs :: xmm2 ()) , (3 , true) => Some (regs :: xmm3 ()) , _ => None , } }
};
}
