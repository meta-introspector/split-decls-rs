// Generated macro for DescriptorType (enum)
macro_rules! Depcrate_segmentationDescriptorType {
() => {
// Module: crate::segmentation
// Provides: {"DescriptorType"}
// Dependencies: {}
# [doc = " Helper enum type to differentiate between the different descriptor types that all end up written in the same field."] # [derive (Debug , Eq , PartialEq)] pub (crate) enum DescriptorType { System64 (SystemDescriptorTypes64) , System32 (SystemDescriptorTypes32) , Data (DataSegmentType) , Code (CodeSegmentType) , }
};
}
