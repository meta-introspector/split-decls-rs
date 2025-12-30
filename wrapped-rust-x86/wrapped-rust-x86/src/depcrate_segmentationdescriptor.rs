// Generated macro for Descriptor (struct)
macro_rules! Depcrate_segmentationDescriptor {
() => {
// Module: crate::segmentation
// Provides: {"Descriptor"}
// Dependencies: {}
# [doc = " Entry for IDT, GDT or LDT. Provides size and location of a segment."] # [doc = ""] # [doc = " See Intel 3a, Section 3.4.5 \"Segment Descriptors\", and Section 3.5.2"] # [derive (Copy , Clone , Debug , Default)] # [repr (packed)] pub struct Descriptor { pub lower : u32 , pub upper : u32 , }
};
}
