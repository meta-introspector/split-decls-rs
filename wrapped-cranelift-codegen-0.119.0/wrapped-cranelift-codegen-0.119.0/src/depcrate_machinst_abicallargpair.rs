// Generated macro for CallArgPair (struct)
macro_rules! Depcrate_machinst_abiCallArgPair {
() => {
// Module: crate::machinst::abi
// Provides: {"CallArgPair"}
// Dependencies: {}
# [doc = " An input argument to a call instruction: the vreg that is used,"] # [doc = " and the preg it is constrained to (per the ABI)."] # [derive (Clone , Debug)] pub struct CallArgPair { # [doc = " The virtual register to use for the argument."] pub vreg : Reg , # [doc = " The real register into which the arg goes."] pub preg : Reg , }
};
}
