// Generated macro for macro_1702 (macro)
macro_rules! Depcrate_segmentationmacro_1702 {
() => {
// Module: crate::segmentation
// Provides: {"macro_1702"}
// Dependencies: {}
bitflags ! { # [doc = " Specifies which element to load into a segment from"] # [doc = " descriptor tables (i.e., is a index to LDT or GDT table"] # [doc = " with some additional flags)."] # [doc = ""] # [doc = " See Intel 3a, Section 3.4.2 \"Segment Selectors\""] pub struct SegmentSelector : u16 { # [doc = " Requestor Privilege Level"] const RPL_0 = 0b00 ; const RPL_1 = 0b01 ; const RPL_2 = 0b10 ; const RPL_3 = 0b11 ; # [doc = " Table Indicator (TI) 0 means GDT is used."] const TI_GDT = 0 << 2 ; # [doc = " Table Indicator (TI) 1 means LDT is used."] const TI_LDT = 1 << 2 ; } }
};
}
