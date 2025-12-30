// Generated macro for AccessType (enum)
macro_rules! Depcrate_data_structures_range_object_mapAccessType {
() => {
// Module: crate::data_structures::range_object_map
// Provides: {"AccessType"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub enum AccessType { # [doc = " The access perfectly overlaps (same offset and range) with the existing allocation"] PerfectlyOverlapping (Position) , # [doc = " The access does not touch any existing allocation"] Empty (Position) , # [doc = " The access overlaps with one or more existing allocations"] ImperfectlyOverlapping (Range < Position >) , }
};
}
