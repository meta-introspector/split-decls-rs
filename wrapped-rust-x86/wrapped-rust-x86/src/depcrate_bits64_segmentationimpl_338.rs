// Generated macro for impl_338 (impl)
macro_rules! Depcrate_bits64_segmentationimpl_338 {
() => {
// Module: crate::bits64::segmentation
// Provides: {"impl_338"}
// Dependencies: {}
impl GateDescriptorBuilder < u64 > for DescriptorBuilder { fn tss_descriptor (base : u64 , limit : u64 , available : bool) -> DescriptorBuilder { let typ = if available { DescriptorType :: System64 (SystemDescriptorTypes64 :: TssAvailable) } else { DescriptorType :: System64 (SystemDescriptorTypes64 :: TssBusy) } ; DescriptorBuilder :: with_base_limit (base , limit) . set_type (typ) } fn call_gate_descriptor (selector : SegmentSelector , offset : u64) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset) . set_type (DescriptorType :: System64 (SystemDescriptorTypes64 :: CallGate)) } fn interrupt_descriptor (selector : SegmentSelector , offset : u64) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset) . set_type (DescriptorType :: System64 (SystemDescriptorTypes64 :: InterruptGate) ,) } fn trap_gate_descriptor (selector : SegmentSelector , offset : u64) -> DescriptorBuilder { DescriptorBuilder :: with_selector_offset (selector , offset) . set_type (DescriptorType :: System64 (SystemDescriptorTypes64 :: TrapGate)) } }
};
}
