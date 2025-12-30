// Generated macro for SystemDescriptorTypes32 (enum)
macro_rules! Depcrate_segmentationSystemDescriptorTypes32 {
() => {
// Module: crate::segmentation
// Provides: {"SystemDescriptorTypes32"}
// Dependencies: {}
# [doc = " System-Segment and Gate-Descriptor Types 32-bit mode."] # [doc = " See also Intel 3a, Table 3-2 System Segment and Gate-Descriptor Types."] # [allow (clippy :: upper_case_acronyms)] # [repr (u8)] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum SystemDescriptorTypes32 { TSSAvailable16 = 0b0001 , LDT = 0b0010 , TSSBusy16 = 0b0011 , CallGate16 = 0b0100 , TaskGate = 0b0101 , InterruptGate16 = 0b0110 , TrapGate16 = 0b0111 , TssAvailable32 = 0b1001 , TssBusy32 = 0b1011 , CallGate32 = 0b1100 , InterruptGate32 = 0b1110 , TrapGate32 = 0b1111 , }
};
}
