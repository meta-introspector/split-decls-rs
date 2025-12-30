// Generated macro for get_intreg_for_retval (function)
macro_rules! Depcrate_isa_x64_abiget_intreg_for_retval {
() => {
// Module: crate::isa::x64::abi
// Provides: {"get_intreg_for_retval"}
// Dependencies: {}
fn get_intreg_for_retval (call_conv : CallConv , flags : & settings :: Flags , intreg_idx : usize , is_last : bool ,) -> Option < Reg > { match call_conv { CallConv :: Tail => match intreg_idx { 0 => Some (regs :: rax ()) , 1 => Some (regs :: rcx ()) , 2 => Some (regs :: rdx ()) , 3 => Some (regs :: rsi ()) , 4 => Some (regs :: rdi ()) , 5 => Some (regs :: r8 ()) , 6 => Some (regs :: r9 ()) , 7 => Some (regs :: r10 ()) , 8 => Some (regs :: r11 ()) , _ => None , } , CallConv :: Fast | CallConv :: Cold | CallConv :: SystemV => match intreg_idx { 0 => Some (regs :: rax ()) , 1 => Some (regs :: rdx ()) , 2 if flags . enable_llvm_abi_extensions () => Some (regs :: rcx ()) , _ => None , } , CallConv :: WindowsFastcall => match intreg_idx { 0 => Some (regs :: rax ()) , 1 => Some (regs :: rdx ()) , _ => None , } , CallConv :: Winch => is_last . then (| | regs :: rax ()) , CallConv :: Probestack => todo ! () , CallConv :: AppleAarch64 => unreachable ! () , } }
};
}
