// Generated macro for CallSite (struct)
macro_rules! Depcrate_machinst_abiCallSite {
() => {
// Module: crate::machinst::abi
// Provides: {"CallSite"}
// Dependencies: {}
# [doc = " ABI object for a callsite."] pub struct CallSite < M : ABIMachineSpec > { # [doc = " The called function's signature."] sig : Sig , # [doc = " All register uses for the callsite, i.e., function args, with"] # [doc = " VReg and the physical register it is constrained to."] uses : CallArgList , # [doc = " All defs for the callsite, i.e., return values."] defs : CallRetList , # [doc = " Call destination."] dest : CallDest , is_tail_call : IsTailCall , # [doc = " Caller's calling convention."] caller_conv : isa :: CallConv , # [doc = " The settings controlling this compilation."] flags : settings :: Flags , _mach : PhantomData < M > , }
};
}
