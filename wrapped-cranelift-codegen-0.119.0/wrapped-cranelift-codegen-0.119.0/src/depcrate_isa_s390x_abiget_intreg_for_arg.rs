// Generated macro for get_intreg_for_arg (function)
macro_rules! Depcrate_isa_s390x_abiget_intreg_for_arg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_intreg_for_arg"}
// Dependencies: {}
fn get_intreg_for_arg (call_conv : isa :: CallConv , idx : usize) -> Option < Reg > { match idx { 0 => Some (regs :: gpr (2)) , 1 => Some (regs :: gpr (3)) , 2 => Some (regs :: gpr (4)) , 3 => Some (regs :: gpr (5)) , 4 => Some (regs :: gpr (6)) , 5 if call_conv == isa :: CallConv :: Tail => Some (regs :: gpr (7)) , _ => None , } }
};
}
