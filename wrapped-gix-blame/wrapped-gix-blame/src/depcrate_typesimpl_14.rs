// Generated macro for impl_14 (impl)
macro_rules! Depcrate_typesimpl_14 {
() => {
// Module: crate::types
// Provides: {"impl_14"}
// Dependencies: {}
# [doc = " Lifecycle"] impl BlameRanges { # [doc = " Create from a single 0-based range."] # [doc = ""] # [doc = " Note that the input range is 1-based inclusive, as used by git, and"] # [doc = " the output is a zero-based `BlameRanges` instance."] pub fn from_one_based_inclusive_range (range : RangeInclusive < u32 >) -> Result < Self , Error > { let zero_based_range = Self :: inclusive_to_zero_based_exclusive (range) ? ; Ok (Self :: PartialFile (vec ! [zero_based_range])) } # [doc = " Create from multiple 0-based ranges."] # [doc = ""] # [doc = " Note that the input ranges are 1-based inclusive, as used by git, and"] # [doc = " the output is a zero-based `BlameRanges` instance."] # [doc = ""] # [doc = " If the input vector is empty, the result will be `WholeFile`."] pub fn from_one_based_inclusive_ranges (ranges : Vec < RangeInclusive < u32 > >) -> Result < Self , Error > { if ranges . is_empty () { return Ok (Self :: WholeFile) ; } let zero_based_ranges = ranges . into_iter () . map (Self :: inclusive_to_zero_based_exclusive) . collect :: < Vec < _ > > () ; let mut result = Self :: PartialFile (vec ! []) ; for range in zero_based_ranges { result . merge_zero_based_exclusive_range (range ?) ; } Ok (result) } # [doc = " Convert a 1-based inclusive range to a 0-based exclusive range."] fn inclusive_to_zero_based_exclusive (range : RangeInclusive < u32 >) -> Result < Range < u32 > , Error > { if range . start () == & 0 { return Err (Error :: InvalidOneBasedLineRange) ; } let start = range . start () - 1 ; let end = * range . end () ; Ok (start .. end) } }
};
}
