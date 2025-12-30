// Generated macro for ReturnCallInfo (struct)
macro_rules! Depcrate_isa_aarch64_instReturnCallInfo {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"ReturnCallInfo"}
// Dependencies: {}
# [doc = " Additional information for `return_call[_ind]` instructions, left out of"] # [doc = " line to lower the size of the `Inst` enum."] # [derive (Clone , Debug)] pub struct ReturnCallInfo < T > { # [doc = " Where this call is going to"] pub dest : T , # [doc = " Arguments to the call instruction."] pub uses : CallArgList , # [doc = " The size of the new stack frame's stack arguments. This is necessary"] # [doc = " for copying the frame over our current frame. It must already be"] # [doc = " allocated on the stack."] pub new_stack_arg_size : u32 , # [doc = " API key to use to restore the return address, if any."] pub key : Option < APIKey > , }
};
}
