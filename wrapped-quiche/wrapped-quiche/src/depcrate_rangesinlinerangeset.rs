// Generated macro for InlineRangeSet (struct)
macro_rules! Depcrate_rangesInlineRangeSet {
() => {
// Module: crate::ranges
// Provides: {"InlineRangeSet"}
// Dependencies: {}
# [doc = " A [`RangeSet`] variant backed by a [`SmallVec`] that is capable of storing"] # [doc = " [`MAX_INLINE_CAPACITY`] of ranges without allocation"] # [derive (Clone , PartialEq , Eq , PartialOrd)] pub struct InlineRangeSet { inner : SmallVec < [(u64 , u64) ; MAX_INLINE_CAPACITY] > , capacity : usize , }
};
}
