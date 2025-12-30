// Generated macro for ReturnCallInfo (struct)
macro_rules! Depcrate_isa_riscv64_instReturnCallInfo {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"ReturnCallInfo"}
// Dependencies: {}
# [doc = " Additional information for `return_call[_ind]` instructions, left out of"] # [doc = " line to lower the size of the `Inst` enum."] # [derive (Clone , Debug)] pub struct ReturnCallInfo < T > { pub dest : T , pub uses : CallArgList , pub new_stack_arg_size : u32 , }
};
}
