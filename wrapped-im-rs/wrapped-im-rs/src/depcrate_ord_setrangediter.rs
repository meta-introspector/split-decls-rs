// Generated macro for RangedIter (struct)
macro_rules! Depcrate_ord_setRangedIter {
() => {
// Module: crate::ord::set
// Provides: {"RangedIter"}
// Dependencies: {}
# [doc = " A ranged iterator over the elements of a set."] # [doc = ""] # [doc = " The only difference from `Iter` is that this one doesn't implement"] # [doc = " `ExactSizeIterator` because we can't know the size of the range without first"] # [doc = " iterating over it to count."] pub struct RangedIter < 'a , A > { it : NodeIter < 'a , Value < A > > , }
};
}
