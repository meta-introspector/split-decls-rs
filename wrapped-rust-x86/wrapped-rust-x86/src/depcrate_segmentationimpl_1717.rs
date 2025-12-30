// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_segmentationimpl_1717 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1717"}
// Dependencies: {}
impl GateDescriptorBuilder < u32 > for DescriptorBuilder { fn tss_descriptor (base : u64 , limit : u64 , available : bool) -> DescriptorBuilder { let typ = match available { true => DescriptorType :: System32 (SystemDescriptorTypes32 :: TssAvailable32) , false => DescriptorType :: System32 (SystemDescriptorTypes32 :: TssBusy32) , } ; DescriptorBuilder :: with_base_limit (base , limit) . set_type (typ) } fn call_gate_descriptor (selector : SegmentSelector , offset : u32) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset . into ()) . set_type (DescriptorType :: System32 (SystemDescriptorTypes32 :: CallGate32) ,) } fn interrupt_descriptor (selector : SegmentSelector , offset : u32) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset . into ()) . set_type (DescriptorType :: System32 (SystemDescriptorTypes32 :: InterruptGate32) ,) } fn trap_gate_descriptor (selector : SegmentSelector , offset : u32) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset . into ()) . set_type (DescriptorType :: System32 (SystemDescriptorTypes32 :: TrapGate32) ,) } }
};
}
