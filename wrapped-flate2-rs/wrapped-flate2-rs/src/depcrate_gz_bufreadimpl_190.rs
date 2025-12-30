// Generated macro for impl_190 (impl)
macro_rules! Depcrate_gz_bufreadimpl_190 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_190"}
// Dependencies: {}
impl < R : BufRead > MultiGzDecoder < R > { # [doc = " Creates a new decoder from the given reader, immediately parsing the"] # [doc = " (first) gzip header. If the gzip stream contains multiple members all will"] # [doc = " be decoded."] pub fn new (r : R) -> MultiGzDecoder < R > { MultiGzDecoder (GzDecoder :: new (r) . multi (true)) } }
};
}
