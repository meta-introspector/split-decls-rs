// Generated macro for LdtDescriptorBuilder (trait)
macro_rules! Depcrate_segmentationLdtDescriptorBuilder {
() => {
// Module: crate::segmentation
// Provides: {"LdtDescriptorBuilder"}
// Dependencies: {}
# [doc = " Trait to define functions that build an architecture specific ldt descriptor."] # [doc = " There is no corresponding ldt descriptor type for 16 bit."] pub trait LdtDescriptorBuilder < Size > { fn ldt_descriptor (base : Size , limit : Size) -> Self ; }
};
}
