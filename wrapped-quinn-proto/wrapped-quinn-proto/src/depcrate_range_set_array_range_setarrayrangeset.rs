// Generated macro for ArrayRangeSet (struct)
macro_rules! Depcrate_range_set_array_range_setArrayRangeSet {
() => {
// Module: crate::range_set::array_range_set
// Provides: {"ArrayRangeSet"}
// Dependencies: {}
# [doc = " A set of u64 values optimized for long runs and random insert/delete/contains"] # [doc = ""] # [doc = " `ArrayRangeSet` uses an array representation, where each array entry represents"] # [doc = " a range."] # [doc = ""] # [doc = " The array-based RangeSet provides 2 benefits:"] # [doc = " - There exists an inline representation, which avoids the need of heap"] # [doc = "   allocating ACK ranges for SentFrames for small ranges."] # [doc = " - Iterating over ranges should usually be faster since there is only"] # [doc = "   a single cache-friendly contiguous range."] # [doc = ""] # [doc = " `ArrayRangeSet` is especially useful for tracking ACK ranges where the amount"] # [doc = " of ranges is usually very low (since ACK numbers are in consecutive fashion"] # [doc = " unless reordering or packet loss occur)."] # [derive (Debug , Default)] pub struct ArrayRangeSet (TinyVec < [Range < u64 > ; ARRAY_RANGE_SET_INLINE_CAPACITY] >) ;
};
}
