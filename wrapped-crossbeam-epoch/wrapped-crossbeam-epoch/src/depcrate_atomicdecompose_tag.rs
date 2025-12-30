// Generated macro for decompose_tag (function)
macro_rules! Depcrate_atomicdecompose_tag {
() => {
// Module: crate::atomic
// Provides: {"decompose_tag"}
// Dependencies: {}
# [doc = " Decomposes a tagged pointer `data` into the pointer and the tag."] # [inline] fn decompose_tag < T : ? Sized + Pointable > (ptr : * mut ()) -> (* mut () , usize) { (map_addr (ptr , | a | a & ! low_bits :: < T > ()) , ptr as usize & low_bits :: < T > () ,) }
};
}
