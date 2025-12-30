// Generated macro for BTreeRangeSet (struct)
macro_rules! Depcrate_rangesBTreeRangeSet {
() => {
// Module: crate::ranges
// Provides: {"BTreeRangeSet"}
// Dependencies: {}
# [doc = " A [`RangeSet`] variant backed by a [`BTreeMap`] that is capable of storing"] # [doc = " an arbitrary number of ranges"] # [derive (Clone , PartialEq , Eq , PartialOrd)] pub struct BTreeRangeSet { inner : BTreeMap < u64 , u64 > , capacity : usize , }
};
}
