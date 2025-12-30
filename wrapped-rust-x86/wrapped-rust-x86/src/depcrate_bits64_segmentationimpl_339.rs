// Generated macro for impl_339 (impl)
macro_rules! Depcrate_bits64_segmentationimpl_339 {
() => {
// Module: crate::bits64::segmentation
// Provides: {"impl_339"}
// Dependencies: {}
impl LdtDescriptorBuilder < u64 > for DescriptorBuilder { fn ldt_descriptor (base : u64 , limit : u64) -> DescriptorBuilder { DescriptorBuilder :: with_base_limit (base , limit) . set_type (DescriptorType :: System64 (SystemDescriptorTypes64 :: LDT)) } }
};
}
