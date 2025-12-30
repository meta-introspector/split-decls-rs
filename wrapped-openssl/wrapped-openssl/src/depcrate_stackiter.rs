// Generated macro for Iter (struct)
macro_rules! Depcrate_stackIter {
() => {
// Module: crate::stack
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the stack's contents."] pub struct Iter < 'a , T : Stackable > { stack : & 'a StackRef < T > , idxs : Range < LenType > , }
};
}
