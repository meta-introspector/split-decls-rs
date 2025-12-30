// Generated macro for DataSegmentType (enum)
macro_rules! Depcrate_segmentationDataSegmentType {
() => {
// Module: crate::segmentation
// Provides: {"DataSegmentType"}
// Dependencies: {}
# [doc = " Data Segment types for descriptors."] # [doc = " See also Intel 3a, Table 3-1 Code- and Data-Segment Types."] # [repr (u8)] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum DataSegmentType { # [doc = " Data Read-Only"] ReadOnly = 0b0000 , # [doc = " Data Read-Only, accessed"] ReadOnlyAccessed = 0b0001 , # [doc = " Data Read/Write"] ReadWrite = 0b0010 , # [doc = " Data Read/Write, accessed"] ReadWriteAccessed = 0b0011 , # [doc = " Data Read-Only, expand-down"] ReadExpand = 0b0100 , # [doc = " Data Read-Only, expand-down, accessed"] ReadExpandAccessed = 0b0101 , # [doc = " Data Read/Write, expand-down"] ReadWriteExpand = 0b0110 , # [doc = " Data Read/Write, expand-down, accessed"] ReadWriteExpandAccessed = 0b0111 , }
};
}
