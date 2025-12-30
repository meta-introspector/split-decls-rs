// Generated macro for impl_1719 (impl)
macro_rules! Depcrate_segmentationimpl_1719 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1719"}
// Dependencies: {}
impl SegmentDescriptorBuilder < u32 > for DescriptorBuilder { fn code_descriptor (base : u32 , limit : u32 , cst : CodeSegmentType) -> DescriptorBuilder { DescriptorBuilder :: with_base_limit (base . into () , limit . into ()) . set_type (DescriptorType :: Code (cst)) } fn data_descriptor (base : u32 , limit : u32 , dst : DataSegmentType) -> DescriptorBuilder { DescriptorBuilder :: with_base_limit (base . into () , limit . into ()) . set_type (DescriptorType :: Data (dst)) } }
};
}
