// Generated macro for EmptyChunkFooter (struct)
macro_rules! DepcrateEmptyChunkFooter {
() => {
// Module: crate
// Provides: {"EmptyChunkFooter"}
// Dependencies: {}
# [doc = " A wrapper type for the canonical, statically allocated empty chunk."] # [doc = ""] # [doc = " For the canonical empty chunk to be `static`, its type must be `Sync`, which"] # [doc = " is the purpose of this wrapper type. This is safe because the empty chunk is"] # [doc = " immutable and never actually modified."] # [repr (transparent)] struct EmptyChunkFooter (ChunkFooter) ;
};
}
