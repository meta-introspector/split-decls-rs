// Generated macro for ToGroupByRange (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_group_byToGroupByRange {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"ToGroupByRange"}
// Dependencies: {}
# [doc = " The trait that provides method `Self::group_by` function which creates a"] # [doc = " `GroupBy` decorated ranged value."] pub trait ToGroupByRange : AsRangedCoord + Sized where Self :: CoordDescType : DiscreteRanged , { # [doc = " Make a grouping ranged value, see the documentation for `GroupBy` for details."] # [doc = ""] # [doc = " - `value`: The number of values we want to group it"] # [doc = " - **return**: The newly created grouping range specification"] fn group_by (self , value : usize) -> GroupBy < < Self as AsRangedCoord > :: CoordDescType > { GroupBy (self . into () , value) } }
};
}
