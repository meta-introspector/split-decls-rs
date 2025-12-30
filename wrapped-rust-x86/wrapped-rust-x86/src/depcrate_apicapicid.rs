// Generated macro for ApicId (enum)
macro_rules! Depcrate_apicApicId {
() => {
// Module: crate::apic
// Provides: {"ApicId"}
// Dependencies: {}
# [doc = " Encodes the id of a core."] # [derive (Debug , Eq , PartialEq , Copy , Clone)] pub enum ApicId { # [doc = " A core destination encoded as an xAPIC ID."] XApic (u8) , # [doc = " A core destination encoded as an x2APIC ID."] X2Apic (u32) , }
};
}
