// Generated macro for ReturnCallInfo (struct)
macro_rules! Depcrate_isa_x64_instReturnCallInfo {
() => {
// Module: crate::isa::x64::inst
// Provides: {"ReturnCallInfo"}
// Dependencies: {}
# [doc = " Out-of-line data for return-calls, to keep the size of `Inst` down."] # [derive (Clone , Debug)] pub struct ReturnCallInfo < T > { # [doc = " Where this call is going."] pub dest : T , # [doc = " The size of the argument area for this return-call, potentially smaller than that of the"] # [doc = " caller, but never larger."] pub new_stack_arg_size : u32 , # [doc = " The in-register arguments and their constraints."] pub uses : CallArgList , # [doc = " A temporary for use when moving the return address."] pub tmp : WritableGpr , }
};
}
