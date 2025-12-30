// Generated macro for RangeListIteratorCoalescer (struct)
macro_rules! Depcrate_iterator_utilsRangeListIteratorCoalescer {
() => {
// Module: crate::iterator_utils
// Provides: {"RangeListIteratorCoalescer"}
// Dependencies: {}
# [doc = " This is an iterator that coalesces adjacent ranges in an iterator over code"] # [doc = " point ranges"] pub (crate) struct RangeListIteratorCoalescer < I , T > { iter : I , peek : Option < CodePointMapRange < T > > , }
};
}
