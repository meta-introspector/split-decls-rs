// Generated macro for ChunkIter (struct)
macro_rules! DepcrateChunkIter {
() => {
// Module: crate
// Provides: {"ChunkIter"}
// Dependencies: {}
# [doc = " An iterator over each chunk of allocated memory that"] # [doc = " an arena has bump allocated into."] # [doc = ""] # [doc = " The chunks are returned ordered by allocation time, with the most recently"] # [doc = " allocated chunk being returned first."] # [doc = ""] # [doc = " The values inside each chunk are also ordered by allocation time, with the most"] # [doc = " recent allocation being earlier in the slice."] # [doc = ""] # [doc = " This struct is created by the [`iter_allocated_chunks`] method on"] # [doc = " [`Bump`]. See that function for a safety description regarding reading from the returned items."] # [doc = ""] # [doc = " [`Bump`]: struct.Bump.html"] # [doc = " [`iter_allocated_chunks`]: struct.Bump.html#method.iter_allocated_chunks"] # [derive (Debug)] pub struct ChunkIter < 'a , const MIN_ALIGN : usize = 1 > { raw : ChunkRawIter < 'a , MIN_ALIGN > , bump : PhantomData < & 'a mut Bump > , }
};
}
