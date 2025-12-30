// Generated macro for CompareBlobs (trait)
macro_rules! Depcrate_index_as_worktree_traitsCompareBlobs {
() => {
// Module: crate::index_as_worktree::traits
// Provides: {"CompareBlobs"}
// Dependencies: {}
# [doc = " Compares the content of two blobs in some way."] pub trait CompareBlobs { # [doc = " Output data produced by [`compare_blobs()`][CompareBlobs::compare_blobs()]."] type Output ; # [doc = " Providing the underlying index `entry`, allow comparing a file in the worktree of size `worktree_blob_size`"] # [doc = " and allow streaming its bytes using `data`."] # [doc = " If this function returns `None` the `entry` and the worktree blob are assumed to be identical."] # [doc = " Use `data` to obtain the data for the blob referred to by `entry`, allowing comparisons of the data itself."] # [doc = " `buf` can be used to store additional data, and it can be assumed to be a cleared buffer."] fn compare_blobs < 'a , 'b > (& mut self , entry : & gix_index :: Entry , worktree_blob_size : u64 , data : impl ReadData < 'a > , buf : & mut Vec < u8 > ,) -> Result < Option < Self :: Output > , Error > ; }
};
}
