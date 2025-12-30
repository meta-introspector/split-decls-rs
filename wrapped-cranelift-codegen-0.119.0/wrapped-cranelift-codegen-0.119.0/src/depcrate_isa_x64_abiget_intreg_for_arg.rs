// Generated macro for get_intreg_for_arg (function)
macro_rules! Depcrate_isa_x64_abiget_intreg_for_arg {
() => {
// Module: crate::isa::x64::abi
// Provides: {"get_intreg_for_arg"}
// Dependencies: {}
fn get_intreg_for_arg (call_conv : CallConv , idx : usize , arg_idx : usize) -> Option < Reg > { let is_fastcall = call_conv == CallConv :: WindowsFastcall ; let i = if is_fastcall { arg_idx } else { idx } ; match (i , is_fastcall) { (0 , false) => Some (regs :: rdi ()) , (1 , false) => Some (regs :: rsi ()) , (2 , false) => Some (regs :: rdx ()) , (3 , false) => Some (regs :: rcx ()) , (4 , false) => Some (regs :: r8 ()) , (5 , false) => Some (regs :: r9 ()) , (0 , true) => Some (regs :: rcx ()) , (1 , true) => Some (regs :: rdx ()) , (2 , true) => Some (regs :: r8 ()) , (3 , true) => Some (regs :: r9 ()) , _ => None , } }
};
}
