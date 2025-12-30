// Generated macro for impl_331 (impl)
macro_rules! Depcrate_iter_rangeimpl_331 {
() => {
// Module: crate::iter_range
// Provides: {"impl_331"}
// Dependencies: {}
impl < K : Into < Vec < u8 > > > IterateBounds for PrefixRange < K > { # [doc = " Converts the prefix range representation into pair of bounds."] # [doc = ""] # [doc = " The conversion assumes lexicographical sorting on `u8` values.  For"] # [doc = " example, `PrefixRange(\"a\")` is equivalent to `\"a\"..\"b\"` range.  Note"] # [doc = " that for some prefixes, either of the bounds may be `None`.  For"] # [doc = " example, an empty prefix is equivalent to a full range (i.e. both bounds"] # [doc = " being `None`)."] fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { let start = self . 0 . into () ; if start . is_empty () { (None , None) } else { let end = next_prefix (& start) ; (Some (start) , end) } } }
};
}
