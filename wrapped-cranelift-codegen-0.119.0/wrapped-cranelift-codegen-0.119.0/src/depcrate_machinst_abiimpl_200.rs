// Generated macro for impl_200 (impl)
macro_rules! Depcrate_machinst_abiimpl_200 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_200"}
// Dependencies: {}
impl < T > CallInfo < T > { # [doc = " Creates an empty set of info with no clobbers/uses/etc with the"] # [doc = " specified ABI"] pub fn empty (dest : T , call_conv : isa :: CallConv) -> CallInfo < T > { CallInfo { dest , uses : smallvec ! [] , defs : smallvec ! [] , clobbers : PRegSet :: empty () , caller_conv : call_conv , callee_conv : call_conv , callee_pop_size : 0 , } } # [doc = " Change the `T` payload on this info to `U`."] pub fn map < U > (self , f : impl FnOnce (T) -> U) -> CallInfo < U > { CallInfo { dest : f (self . dest) , uses : self . uses , defs : self . defs , clobbers : self . clobbers , caller_conv : self . caller_conv , callee_conv : self . callee_conv , callee_pop_size : self . callee_pop_size , } } }
};
}
