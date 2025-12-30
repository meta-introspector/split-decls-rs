// Generated macro for CallInfo (struct)
macro_rules! Depcrate_machinst_abiCallInfo {
() => {
// Module: crate::machinst::abi
// Provides: {"CallInfo"}
// Dependencies: {}
# [doc = " Out-of-line data for calls, to keep the size of `Inst` down."] # [derive (Clone , Debug)] pub struct CallInfo < T > { # [doc = " Receiver of this call"] pub dest : T , # [doc = " Register uses of this call."] pub uses : CallArgList , # [doc = " Register defs of this call."] pub defs : CallRetList , # [doc = " Registers clobbered by this call, as per its calling convention."] pub clobbers : PRegSet , # [doc = " The calling convention of the callee."] pub callee_conv : isa :: CallConv , # [doc = " The calling convention of the caller."] pub caller_conv : isa :: CallConv , # [doc = " The number of bytes that the callee will pop from the stack for the"] # [doc = " caller, if any. (Used for popping stack arguments with the `tail`"] # [doc = " calling convention.)"] pub callee_pop_size : u32 , }
};
}
