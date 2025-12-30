// Generated macro for ClassUnicodeIter (struct)
macro_rules! Depcrate_hirClassUnicodeIter {
() => {
// Module: crate::hir
// Provides: {"ClassUnicodeIter"}
// Dependencies: {}
# [doc = " An iterator over all ranges in a Unicode character class."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the underlying class."] # [derive (Debug)] pub struct ClassUnicodeIter < 'a > (IntervalSetIter < 'a , ClassUnicodeRange >) ;
};
}
