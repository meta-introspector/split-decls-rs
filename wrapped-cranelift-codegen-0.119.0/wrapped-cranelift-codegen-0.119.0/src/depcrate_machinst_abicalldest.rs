// Generated macro for CallDest (enum)
macro_rules! Depcrate_machinst_abiCallDest {
() => {
// Module: crate::machinst::abi
// Provides: {"CallDest"}
// Dependencies: {}
# [doc = " Destination for a call."] # [derive (Debug , Clone)] pub enum CallDest { # [doc = " Call to an ExtName (named function symbol)."] ExtName (ir :: ExternalName , RelocDistance) , # [doc = " Indirect call to a function pointer in a register."] Reg (Reg) , }
};
}
