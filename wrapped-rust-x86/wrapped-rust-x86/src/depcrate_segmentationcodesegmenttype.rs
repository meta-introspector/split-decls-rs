// Generated macro for CodeSegmentType (enum)
macro_rules! Depcrate_segmentationCodeSegmentType {
() => {
// Module: crate::segmentation
// Provides: {"CodeSegmentType"}
// Dependencies: {}
# [doc = " Code Segment types for descriptors."] # [doc = " See also Intel 3a, Table 3-1 Code- and Data-Segment Types."] # [repr (u8)] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum CodeSegmentType { # [doc = " Code Execute-Only"] Execute = 0b1000 , # [doc = " Code Execute-Only, accessed"] ExecuteAccessed = 0b1001 , # [doc = " Code Execute/Read"] ExecuteRead = 0b1010 , # [doc = " Code Execute/Read, accessed"] ExecuteReadAccessed = 0b1011 , # [doc = " Code Execute-Only, conforming"] ExecuteConforming = 0b1100 , # [doc = " Code Execute-Only, conforming, accessed"] ExecuteConformingAccessed = 0b1101 , # [doc = " Code Execute/Read, conforming"] ExecuteReadConforming = 0b1110 , # [doc = " Code Execute/Read, conforming, accessed"] ExecuteReadConformingAccessed = 0b1111 , }
};
}
