// Generated macro for DimRange (trait)
macro_rules! Depcrate_base_matrix_viewDimRange {
() => {
// Module: crate::base::matrix_view
// Provides: {"DimRange"}
// Dependencies: {}
# [doc = " A range with a size that may be known at compile-time."] # [doc = ""] # [doc = " This may be:"] # [doc = " * A single `usize` index, e.g., `4`"] # [doc = " * A left-open range `std::ops::RangeTo`, e.g., `.. 4`"] # [doc = " * A right-open range `std::ops::RangeFrom`, e.g., `4 ..`"] # [doc = " * A full range `std::ops::RangeFull`, e.g., `..`"] pub trait DimRange < D : Dim > { # [doc = " Type of the range size. May be a type-level integer."] type Size : Dim ; # [doc = " The start index of the range."] fn begin (& self , shape : D) -> usize ; # [doc = " The index immediately after the last index inside of the range."] fn end (& self , shape : D) -> usize ; # [doc = " The number of elements of the range, i.e., `self.end - self.begin`."] fn size (& self , shape : D) -> Self :: Size ; }
};
}
