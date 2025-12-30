// Generated macro for Descriptor64 (struct)
macro_rules! Depcrate_bits64_segmentationDescriptor64 {
() => {
// Module: crate::bits64::segmentation
// Provides: {"Descriptor64"}
// Dependencies: {}
# [doc = " Entry for IDT, GDT or LDT."] # [doc = ""] # [doc = " See Intel 3a, Section 3.4.5 \"Segment Descriptors\", and Section 3.5.2"] # [doc = " \"Segment Descriptor Tables in IA-32e Mode\", especially Figure 3-8."] # [derive (Copy , Clone , Debug , Default)] # [repr (C , packed)] pub struct Descriptor64 { desc32 : Descriptor , lower : u32 , upper : u32 , }
};
}
