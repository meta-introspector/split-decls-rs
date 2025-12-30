// Generated macro for IterMut (struct)
macro_rules! Depcrate_stackIterMut {
() => {
// Module: crate::stack
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the stack's contents."] pub struct IterMut < 'a , T : Stackable > { stack : & 'a mut StackRef < T > , idxs : Range < LenType > , }
};
}
