// Generated macro for ChunksMut (struct)
macro_rules! Depcrate_vectorChunksMut {
() => {
// Module: crate::vector
// Provides: {"ChunksMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the leaf nodes of a vector."] # [doc = ""] # [doc = " To obtain one, use [`Vector::chunks_mut()`][chunks_mut]."] # [doc = ""] # [doc = " [chunks_mut]: enum.Vector.html#method.chunks_mut"] pub struct ChunksMut < 'a , A > { focus : FocusMut < 'a , A > , front_index : usize , back_index : usize , }
};
}
