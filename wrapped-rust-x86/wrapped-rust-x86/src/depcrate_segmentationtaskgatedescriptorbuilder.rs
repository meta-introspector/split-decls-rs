// Generated macro for TaskGateDescriptorBuilder (trait)
macro_rules! Depcrate_segmentationTaskGateDescriptorBuilder {
() => {
// Module: crate::segmentation
// Provides: {"TaskGateDescriptorBuilder"}
// Dependencies: {}
# [doc = " Trait to implement for building a task-gate (this descriptor is not implemented for 64-bit systems since"] # [doc = " Hardware task switches are not supported in IA-32e mode.)."] pub trait TaskGateDescriptorBuilder { fn task_gate_descriptor (selector : SegmentSelector) -> Self ; }
};
}
