// Generated macro for Chunks (struct)
macro_rules! Depcrate_vectorChunks {
() => {
// Module: crate::vector
// Provides: {"Chunks"}
// Dependencies: {}
# [doc = " An iterator over the leaf nodes of a vector."] # [doc = ""] # [doc = " To obtain one, use [`Vector::chunks()`][chunks]."] # [doc = ""] # [doc = " [chunks]: enum.Vector.html#method.chunks"] pub struct Chunks < 'a , A > { focus : Focus < 'a , A > , front_index : usize , back_index : usize , }
};
}
