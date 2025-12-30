// Generated macro for impl_1720 (impl)
macro_rules! Depcrate_segmentationimpl_1720 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1720"}
// Dependencies: {}
impl LdtDescriptorBuilder < u32 > for DescriptorBuilder { fn ldt_descriptor (base : u32 , limit : u32) -> DescriptorBuilder { DescriptorBuilder :: with_base_limit (base . into () , limit . into ()) . set_type (DescriptorType :: System32 (SystemDescriptorTypes32 :: LDT)) } }
};
}
