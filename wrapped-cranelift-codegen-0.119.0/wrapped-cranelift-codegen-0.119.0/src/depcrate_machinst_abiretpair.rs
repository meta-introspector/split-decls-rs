// Generated macro for RetPair (struct)
macro_rules! Depcrate_machinst_abiRetPair {
() => {
// Module: crate::machinst::abi
// Provides: {"RetPair"}
// Dependencies: {}
# [doc = " A type used by backends to track return register binding info in the \"ret\""] # [doc = " pseudoinst. The pseudoinst holds a vec of `RetPair` structs."] # [derive (Clone , Debug)] pub struct RetPair { # [doc = " The vreg that is returned by this pseudionst."] pub vreg : Reg , # [doc = " The preg that the arg is returned through; this constrains the vreg's"] # [doc = " placement at the pseudoinst."] pub preg : Reg , }
};
}
