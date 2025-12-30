// Generated macro for GateDescriptorBuilder (trait)
macro_rules! Depcrate_segmentationGateDescriptorBuilder {
() => {
// Module: crate::segmentation
// Provides: {"GateDescriptorBuilder"}
// Dependencies: {}
# [doc = " Trait that defines the architecture specific functions for building various system segment descriptors"] # [doc = " which are available on all 16, 32, and 64 bits."] pub trait GateDescriptorBuilder < Size > { fn tss_descriptor (base : u64 , limit : u64 , available : bool) -> Self ; fn call_gate_descriptor (selector : SegmentSelector , offset : Size) -> Self ; fn interrupt_descriptor (selector : SegmentSelector , offset : Size) -> Self ; fn trap_gate_descriptor (selector : SegmentSelector , offset : Size) -> Self ; }
};
}
