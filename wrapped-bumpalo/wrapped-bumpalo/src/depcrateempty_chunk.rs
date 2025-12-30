// Generated macro for EMPTY_CHUNK (static)
macro_rules! DepcrateEMPTY_CHUNK {
() => {
// Module: crate
// Provides: {"EMPTY_CHUNK"}
// Dependencies: {}
static EMPTY_CHUNK : EmptyChunkFooter = EmptyChunkFooter (ChunkFooter { layout : Layout :: new :: < ChunkFooter > () , data : unsafe { NonNull :: new_unchecked (& EMPTY_CHUNK as * const EmptyChunkFooter as * mut u8) } , ptr : Cell :: new (unsafe { NonNull :: new_unchecked (& EMPTY_CHUNK as * const EmptyChunkFooter as * mut u8) }) , prev : Cell :: new (unsafe { NonNull :: new_unchecked (& EMPTY_CHUNK as * const EmptyChunkFooter as * mut ChunkFooter) }) , allocated_bytes : 0 , }) ;
};
}
