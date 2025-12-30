// Generated macro for impl_45 (impl)
macro_rules! Depcrate_bufreadimpl_45 {
() => {
// Module: crate::bufread
// Provides: {"impl_45"}
// Dependencies: {}
impl < R : BufRead > XzEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] # [doc = ""] # [doc = " The `level` argument here is typically 0-9 with 6 being a good default."] # [inline] pub fn new (r : R , level : u32) -> XzEncoder < R > { let stream = Stream :: new_easy_encoder (level , Check :: Crc64) . unwrap () ; XzEncoder :: new_stream (r , stream) } # [doc = " Creates a new parallel encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] # [doc = ""] # [doc = " The `level` argument here is typically 0-9 with 6 being a good default."] # [cfg (feature = "parallel")] pub fn new_parallel (r : R , level : u32) -> XzEncoder < R > { let stream = MtStreamBuilder :: new () . preset (level) . check (Check :: Crc64) . threads (num_cpus :: get () as u32) . encoder () . unwrap () ; Self :: new_stream (r , stream) } # [doc = " Creates a new encoder with a custom `Stream`."] # [doc = ""] # [doc = " The `Stream` can be pre-configured for multithreaded encoding, different"] # [doc = " compression options/tuning, etc."] # [inline] pub fn new_stream (r : R , stream : Stream) -> XzEncoder < R > { XzEncoder { obj : r , data : stream , } } }
};
}
