// Generated macro for SegmentDescriptorBuilder (trait)
macro_rules! Depcrate_segmentationSegmentDescriptorBuilder {
() => {
// Module: crate::segmentation
// Provides: {"SegmentDescriptorBuilder"}
// Dependencies: {}
# [doc = " Trait to define functions that build architecture specific code and data descriptors."] pub trait SegmentDescriptorBuilder < Size > { fn code_descriptor (base : Size , limit : Size , cst : CodeSegmentType) -> Self ; fn data_descriptor (base : Size , limit : Size , dst : DataSegmentType) -> Self ; }
};
}
