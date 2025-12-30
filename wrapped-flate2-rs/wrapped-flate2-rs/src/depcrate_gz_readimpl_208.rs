// Generated macro for impl_208 (impl)
macro_rules! Depcrate_gz_readimpl_208 {
() => {
// Module: crate::gz::read
// Provides: {"impl_208"}
// Dependencies: {}
impl < R : Read > GzDecoder < R > { # [doc = " Creates a new decoder from the given reader, immediately parsing the"] # [doc = " gzip header."] pub fn new (r : R) -> GzDecoder < R > { GzDecoder { inner : bufread :: GzDecoder :: new (BufReader :: new (r)) , } } }
};
}
