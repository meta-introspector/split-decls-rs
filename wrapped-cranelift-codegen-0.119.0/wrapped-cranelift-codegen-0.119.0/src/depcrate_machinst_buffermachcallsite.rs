// Generated macro for MachCallSite (struct)
macro_rules! Depcrate_machinst_bufferMachCallSite {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachCallSite"}
// Dependencies: {}
# [doc = " A call site record resulting from a compilation."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct MachCallSite { # [doc = " The offset of the call's return address, *relative to the containing section*."] pub ret_addr : CodeOffset , }
};
}
