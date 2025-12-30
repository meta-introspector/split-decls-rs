// Generated macro for Replace (struct)
macro_rules! Depcrate_range_set_btree_range_setReplace {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"Replace"}
// Dependencies: {}
# [doc = " Iterator returned by `RangeSet::replace`"] pub struct Replace < 'a > { set : & 'a mut RangeSet , # [doc = " Portion of the intersection arising from a range beginning at or before the newly inserted"] # [doc = " range"] pred : Option < Range < u64 > > , # [doc = " Union of the input range and all ranges that have been visited by the iterator so far"] range : Range < u64 > , }
};
}
