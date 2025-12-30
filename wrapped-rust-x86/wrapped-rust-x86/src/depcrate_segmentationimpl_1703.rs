// Generated macro for impl_1703 (impl)
macro_rules! Depcrate_segmentationimpl_1703 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1703"}
// Dependencies: {}
impl SegmentSelector { # [doc = " Create a new SegmentSelector"] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * `index` - index in GDT or LDT array."] # [doc = "  * `rpl` - Requested privilege level of the selector"] pub const fn new (index : u16 , rpl : Ring) -> SegmentSelector { SegmentSelector { bits : index << 3 | (rpl as u16) , } } # [doc = " Returns segment selector's index in GDT or LDT."] pub const fn index (& self) -> u16 { self . bits >> 3 } # [doc = " Make a new segment selector from a untyped u16 value."] pub const fn from_raw (bits : u16) -> SegmentSelector { SegmentSelector { bits } } }
};
}
