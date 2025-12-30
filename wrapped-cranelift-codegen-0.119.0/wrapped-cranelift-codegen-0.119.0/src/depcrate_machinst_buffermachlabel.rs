// Generated macro for MachLabel (struct)
macro_rules! Depcrate_machinst_bufferMachLabel {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachLabel"}
// Dependencies: {}
# [doc = " A label refers to some offset in a `MachBuffer`. It may not be resolved at"] # [doc = " the point at which it is used by emitted code; the buffer records \"fixups\""] # [doc = " for references to the label, and will come back and patch the code"] # [doc = " appropriately when the label's location is eventually known."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct MachLabel (u32) ;
};
}
