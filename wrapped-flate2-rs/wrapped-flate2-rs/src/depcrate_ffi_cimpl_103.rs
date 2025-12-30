// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ffi_cimpl_103 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_103"}
// Dependencies: {}
impl Default for StreamWrapper { fn default () -> StreamWrapper { StreamWrapper { inner : Box :: into_raw (Box :: new (mz_stream { next_in : ptr :: null_mut () , avail_in : 0 , total_in : 0 , next_out : ptr :: null_mut () , avail_out : 0 , total_out : 0 , msg : ptr :: null_mut () , adler : 0 , data_type : 0 , reserved : 0 , opaque : ptr :: null_mut () , state : ptr :: null_mut () , # [cfg (any (feature = "zlib-ng" , all (not (feature = "cloudflare_zlib") , not (feature = "zlib-ng") , not (feature = "zlib-rs"))))] zalloc : allocator :: zalloc , # [cfg (any (feature = "zlib-ng" , all (not (feature = "cloudflare_zlib") , not (feature = "zlib-ng") , not (feature = "zlib-rs"))))] zfree : allocator :: zfree , # [cfg (all (feature = "cloudflare_zlib" , not (feature = "zlib-rs") , not (feature = "zlib-ng")) ,)] zalloc : Some (allocator :: zalloc) , # [cfg (all (feature = "cloudflare_zlib" , not (feature = "zlib-rs") , not (feature = "zlib-ng")) ,)] zfree : Some (allocator :: zfree) , # [cfg (all (feature = "zlib-rs" , not (feature = "zlib-ng")))] zalloc : None , # [cfg (all (feature = "zlib-rs" , not (feature = "zlib-ng")))] zfree : None , })) , } } }
};
}
