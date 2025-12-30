// Generated macro for MachTrap (struct)
macro_rules! Depcrate_machinst_bufferMachTrap {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachTrap"}
// Dependencies: {}
# [doc = " A trap record resulting from a compilation."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct MachTrap { # [doc = " The offset at which the trap instruction occurs, *relative to the"] # [doc = " containing section*."] pub offset : CodeOffset , # [doc = " The trap code."] pub code : TrapCode , }
};
}
