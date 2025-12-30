// Generated macro for impl_49 (impl)
macro_rules! Depcrate_bufreadimpl_49 {
() => {
// Module: crate::bufread
// Provides: {"impl_49"}
// Dependencies: {}
impl < R : BufRead > XzDecoder < R > { # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream."] # [inline] pub fn new (r : R) -> XzDecoder < R > { let stream = Stream :: new_stream_decoder (u64 :: MAX , 0) . unwrap () ; XzDecoder :: new_stream (r , stream) } # [doc = " Creates a new parallel decoder which will decompress data read from the given"] # [doc = " stream."] # [cfg (feature = "parallel")] pub fn new_parallel (r : R) -> Self { let stream = MtStreamBuilder :: new () . memlimit_stop (u64 :: MAX) . threads (num_cpus :: get () as u32) . decoder () . unwrap () ; Self :: new_stream (r , stream) } # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " input. All the concatenated xz streams from input will be consumed."] # [inline] pub fn new_multi_decoder (r : R) -> XzDecoder < R > { let stream = Stream :: new_auto_decoder (u64 :: MAX , liblzma_sys :: LZMA_CONCATENATED) . unwrap () ; XzDecoder :: new_stream (r , stream) } # [doc = " Creates a new decoder with a custom `Stream`."] # [doc = ""] # [doc = " The `Stream` can be pre-configured for various checks, different"] # [doc = " decompression options/tuning, etc."] # [inline] pub fn new_stream (r : R , stream : Stream) -> XzDecoder < R > { XzDecoder { obj : r , data : stream , } } }
};
}
