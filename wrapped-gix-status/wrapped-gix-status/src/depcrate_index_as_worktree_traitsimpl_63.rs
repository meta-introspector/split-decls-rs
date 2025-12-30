// Generated macro for impl_63 (impl)
macro_rules! Depcrate_index_as_worktree_traitsimpl_63 {
() => {
// Module: crate::index_as_worktree::traits
// Provides: {"impl_63"}
// Dependencies: {}
impl CompareBlobs for HashEq { type Output = ObjectId ; fn compare_blobs < 'a , 'b > (& mut self , entry : & Entry , _worktree_blob_size : u64 , data : impl ReadData < 'a > , buf : & mut Vec < u8 > ,) -> Result < Option < Self :: Output > , Error > { let mut stream = data . stream_worktree_file () ? ; match stream . as_bytes () { Some (buffer) => { let file_hash = gix_object :: compute_hash (entry . id . kind () , gix_object :: Kind :: Blob , buffer) . map_err (gix_hash :: io :: Error :: from) ? ; Ok ((entry . id != file_hash) . then_some (file_hash)) } None => { let file_hash = match stream . size () { None => { stream . read_to_end (buf) . map_err (gix_hash :: io :: Error :: from) ? ; gix_object :: compute_hash (entry . id . kind () , gix_object :: Kind :: Blob , buf) . map_err (gix_hash :: io :: Error :: from) ? } Some (len) => gix_object :: compute_stream_hash (entry . id . kind () , gix_object :: Kind :: Blob , & mut stream , len , & mut gix_features :: progress :: Discard , & AtomicBool :: default () ,) ? , } ; Ok ((entry . id != file_hash) . then_some (file_hash)) } } } }
};
}
