// Generated macro for CallRetPair (struct)
macro_rules! Depcrate_machinst_abiCallRetPair {
() => {
// Module: crate::machinst::abi
// Provides: {"CallRetPair"}
// Dependencies: {}
# [doc = " An output return value from a call instruction: the vreg that is"] # [doc = " defined, and the preg it is constrained to (per the ABI)."] # [derive (Clone , Debug)] pub struct CallRetPair { # [doc = " The virtual register to define from this return value."] pub vreg : Writable < Reg > , # [doc = " The real register from which the return value is read."] pub preg : Reg , }
};
}
