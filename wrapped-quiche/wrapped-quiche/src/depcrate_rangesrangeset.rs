// Generated macro for RangeSet (enum)
macro_rules! Depcrate_rangesRangeSet {
() => {
// Module: crate::ranges
// Provides: {"RangeSet"}
// Dependencies: {}
# [doc = " A sorted collection of non overlapping [`u64`] ranges"] # [derive (Clone , PartialEq , Eq , PartialOrd)] pub enum RangeSet { Inline (InlineRangeSet) , BTree (BTreeRangeSet) , }
};
}
