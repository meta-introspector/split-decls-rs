// Generated macro for ChunkRawIter (struct)
macro_rules! DepcrateChunkRawIter {
() => {
// Module: crate
// Provides: {"ChunkRawIter"}
// Dependencies: {}
# [doc = " An iterator over raw pointers to chunks of allocated memory that this"] # [doc = " arena has bump allocated into."] # [doc = ""] # [doc = " See [`ChunkIter`] for details regarding the returned chunks."] # [doc = ""] # [doc = " This struct is created by the [`iter_allocated_chunks_raw`] method on"] # [doc = " [`Bump`]. See that function for a safety description regarding reading from"] # [doc = " the returned items."] # [doc = ""] # [doc = " [`Bump`]: struct.Bump.html"] # [doc = " [`iter_allocated_chunks_raw`]: struct.Bump.html#method.iter_allocated_chunks_raw"] # [derive (Debug)] pub struct ChunkRawIter < 'a , const MIN_ALIGN : usize = 1 > { footer : NonNull < ChunkFooter > , bump : PhantomData < & 'a Bump < MIN_ALIGN > > , }
};
}
