// Generated macro for compose_tag (function)
macro_rules! Depcrate_atomiccompose_tag {
() => {
// Module: crate::atomic
// Provides: {"compose_tag"}
// Dependencies: {}
# [doc = " Given a tagged pointer `data`, returns the same pointer, but tagged with `tag`."] # [doc = ""] # [doc = " `tag` is truncated to fit into the unused bits of the pointer to `T`."] # [inline] fn compose_tag < T : ? Sized + Pointable > (ptr : * mut () , tag : usize) -> * mut () { map_addr (ptr , | a | (a & ! low_bits :: < T > ()) | (tag & low_bits :: < T > ())) }
};
}
