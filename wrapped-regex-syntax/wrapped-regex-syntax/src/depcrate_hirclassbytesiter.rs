// Generated macro for ClassBytesIter (struct)
macro_rules! Depcrate_hirClassBytesIter {
() => {
// Module: crate::hir
// Provides: {"ClassBytesIter"}
// Dependencies: {}
# [doc = " An iterator over all ranges in a byte character class."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the underlying class."] # [derive (Debug)] pub struct ClassBytesIter < 'a > (IntervalSetIter < 'a , ClassBytesRange >) ;
};
}
