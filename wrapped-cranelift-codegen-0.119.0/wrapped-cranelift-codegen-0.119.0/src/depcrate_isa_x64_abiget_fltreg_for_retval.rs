// Generated macro for get_fltreg_for_retval (function)
macro_rules! Depcrate_isa_x64_abiget_fltreg_for_retval {
() => {
// Module: crate::isa::x64::abi
// Provides: {"get_fltreg_for_retval"}
// Dependencies: {}
fn get_fltreg_for_retval (call_conv : CallConv , fltreg_idx : usize , is_last : bool) -> Option < Reg > { match call_conv { CallConv :: Tail => match fltreg_idx { 0 => Some (regs :: xmm0 ()) , 1 => Some (regs :: xmm1 ()) , 2 => Some (regs :: xmm2 ()) , 3 => Some (regs :: xmm3 ()) , 4 => Some (regs :: xmm4 ()) , 5 => Some (regs :: xmm5 ()) , 6 => Some (regs :: xmm6 ()) , 7 => Some (regs :: xmm7 ()) , _ => None , } , CallConv :: Fast | CallConv :: Cold | CallConv :: SystemV => match fltreg_idx { 0 => Some (regs :: xmm0 ()) , 1 => Some (regs :: xmm1 ()) , _ => None , } , CallConv :: WindowsFastcall => match fltreg_idx { 0 => Some (regs :: xmm0 ()) , _ => None , } , CallConv :: Winch => is_last . then (| | regs :: xmm0 ()) , CallConv :: Probestack => todo ! () , CallConv :: AppleAarch64 => unreachable ! () , } }
};
}
