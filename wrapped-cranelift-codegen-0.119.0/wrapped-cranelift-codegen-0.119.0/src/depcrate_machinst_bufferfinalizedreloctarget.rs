// Generated macro for FinalizedRelocTarget (enum)
macro_rules! Depcrate_machinst_bufferFinalizedRelocTarget {
() => {
// Module: crate::machinst::buffer
// Provides: {"FinalizedRelocTarget"}
// Dependencies: {}
# [doc = " A Relocation target"] # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub enum FinalizedRelocTarget { # [doc = " Points to an [ExternalName] outside the current function."] ExternalName (ExternalName) , # [doc = " Points to a [CodeOffset] from the start of the current function."] Func (CodeOffset) , }
};
}
