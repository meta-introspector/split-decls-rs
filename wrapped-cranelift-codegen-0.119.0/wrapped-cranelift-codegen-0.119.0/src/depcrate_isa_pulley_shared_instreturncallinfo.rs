// Generated macro for ReturnCallInfo (struct)
macro_rules! Depcrate_isa_pulley_shared_instReturnCallInfo {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"ReturnCallInfo"}
// Dependencies: {}
# [doc = " Out-of-line data for return-calls, to keep the size of `Inst` down."] # [derive (Clone , Debug)] pub struct ReturnCallInfo < T > { # [doc = " Where this call is going."] pub dest : T , # [doc = " The size of the argument area for this return-call, potentially smaller"] # [doc = " than that of the caller, but never larger."] pub new_stack_arg_size : u32 , # [doc = " The in-register arguments and their constraints."] pub uses : CallArgList , }
};
}
