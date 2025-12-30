// Generated macro for RelocTarget (enum)
macro_rules! Depcrate_machinst_bufferRelocTarget {
() => {
// Module: crate::machinst::buffer
// Provides: {"RelocTarget"}
// Dependencies: {}
# [doc = " A Relocation target"] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum RelocTarget { # [doc = " Points to an [ExternalName] outside the current function."] ExternalName (ExternalName) , # [doc = " Points to a [MachLabel] inside this function."] # [doc = " This is different from [MachLabelFixup] in that both the relocation and the"] # [doc = " label will be emitted and are only resolved at link time."] # [doc = ""] # [doc = " There is no reason to prefer this over [MachLabelFixup] unless the ABI requires it."] Label (MachLabel) , }
};
}
