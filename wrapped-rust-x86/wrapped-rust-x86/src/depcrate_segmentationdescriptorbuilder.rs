// Generated macro for DescriptorBuilder (struct)
macro_rules! Depcrate_segmentationDescriptorBuilder {
() => {
// Module: crate::segmentation
// Provides: {"DescriptorBuilder"}
// Dependencies: {}
# [doc = " Makes building descriptors easier (hopefully)."] # [derive (Debug)] pub struct DescriptorBuilder { # [doc = " The base defines the location of byte 0 of the segment within the 4-GByte linear address space."] # [doc = " The limit is the size of the range covered by the segment. Really a 20bit value."] pub (crate) base_limit : Option < (u64 , u64) > , # [doc = " Alternative to base_limit we use a selector that points to a segment and an an offset for certain descriptors."] pub (crate) selector_offset : Option < (SegmentSelector , u64) > , # [doc = " Descriptor type"] pub (crate) typ : Option < DescriptorType > , # [doc = " Specifies the privilege level of the segment. The privilege level can range from 0 to 3, with 0 being the most privileged level."] pub (crate) dpl : Option < Ring > , # [doc = " Indicates whether the segment is present in memory (set) or not present (clear)."] pub (crate) present : bool , # [doc = " Available for use by system software"] pub (crate) avl : bool , # [doc = " Default operation size"] pub (crate) db : bool , # [doc = " Determines the scaling of the segment limit field. When the granularity flag is clear, the segment limit is interpreted in byte units; when flag is set, the segment limit is interpreted in 4-KByte units."] pub (crate) limit_granularity_4k : bool , # [doc = " 64-bit code segment (IA-32e mode only)"] pub (crate) l : bool , # [doc = " Interrupt stack table (IST) selector (IA-32e mode only)"] pub (crate) ist : u8 , }
};
}
