// Generated macro for impl_213 (impl)
macro_rules! Depcrate_gz_readimpl_213 {
() => {
// Module: crate::gz::read
// Provides: {"impl_213"}
// Dependencies: {}
impl < R : Read > MultiGzDecoder < R > { # [doc = " Creates a new decoder from the given reader, immediately parsing the"] # [doc = " (first) gzip header. If the gzip stream contains multiple members all will"] # [doc = " be decoded."] pub fn new (r : R) -> MultiGzDecoder < R > { MultiGzDecoder { inner : bufread :: MultiGzDecoder :: new (BufReader :: new (r)) , } } }
};
}
