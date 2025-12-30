// Generated macro for ArgPair (struct)
macro_rules! Depcrate_machinst_abiArgPair {
() => {
// Module: crate::machinst::abi
// Provides: {"ArgPair"}
// Dependencies: {}
# [doc = " A type used by backends to track argument-binding info in the \"args\""] # [doc = " pseudoinst. The pseudoinst holds a vec of `ArgPair` structs."] # [derive (Clone , Debug)] pub struct ArgPair { # [doc = " The vreg that is defined by this args pseudoinst."] pub vreg : Writable < Reg > , # [doc = " The preg that the arg arrives in; this constrains the vreg's"] # [doc = " placement at the pseudoinst."] pub preg : Reg , }
};
}
