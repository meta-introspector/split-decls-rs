// Generated macro for SystemDescriptorTypes64 (enum)
macro_rules! Depcrate_segmentationSystemDescriptorTypes64 {
() => {
// Module: crate::segmentation
// Provides: {"SystemDescriptorTypes64"}
// Dependencies: {}
# [doc = " System-Segment and Gate-Descriptor Types 64-bit mode"] # [doc = " See also Intel 3a, Table 3-2 System Segment and Gate-Descriptor Types."] # [allow (clippy :: upper_case_acronyms)] # [repr (u8)] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum SystemDescriptorTypes64 { LDT = 0b0010 , TssAvailable = 0b1001 , TssBusy = 0b1011 , CallGate = 0b1100 , InterruptGate = 0b1110 , TrapGate = 0b1111 , }
};
}
