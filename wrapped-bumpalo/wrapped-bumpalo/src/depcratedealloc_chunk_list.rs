// Generated macro for dealloc_chunk_list (function)
macro_rules! Depcratedealloc_chunk_list {
() => {
// Module: crate
// Provides: {"dealloc_chunk_list"}
// Dependencies: {}
# [inline] unsafe fn dealloc_chunk_list (mut footer : NonNull < ChunkFooter >) { while ! footer . as_ref () . is_empty () { let f = footer ; footer = f . as_ref () . prev . get () ; dealloc (f . as_ref () . data . as_ptr () , f . as_ref () . layout) ; } }
};
}
