// Generated macro for read_value_ref_with_max_depth (function)
macro_rules! Depcrate_decode_value_refread_value_ref_with_max_depth {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_value_ref_with_max_depth"}
// Dependencies: {}
# [doc = " Attempts to read the data from the given reader until either a complete MessagePack value"] # [doc = " decoded or an error detected."] # [doc = ""] # [doc = " Returns either a non-owning `ValueRef`, which borrows the buffer from the given reader or an"] # [doc = " error."] # [doc = ""] # [doc = " See [`read_value_ref`] for more information on how to use this function. This variant allows"] # [doc = " you to specify the maximum recursion depth, if [`MAX_DEPTH`](super::MAX_DEPTH) is too high."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Same as [`read_value_ref`], using the `max_depth` parameter in place of"] # [doc = " [`MAX_DEPTH`](super::MAX_DEPTH)."] # [inline (never)] pub fn read_value_ref_with_max_depth < 'a , R > (rd : & mut R , max_depth : usize) -> Result < ValueRef < 'a > , Error > where R : BorrowRead < 'a > { read_value_ref_inner (rd , max_depth . min (u16 :: MAX as _) as u16) }
};
}
