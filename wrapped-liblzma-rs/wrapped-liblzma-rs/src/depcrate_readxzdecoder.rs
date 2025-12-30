// Generated macro for XzDecoder (struct)
macro_rules! Depcrate_readXzDecoder {
() => {
// Module: crate::read
// Provides: {"XzDecoder"}
// Dependencies: {}
# [doc = " A decompression stream which wraps a compressed stream of data. Decompressed"] # [doc = " data will be read from the stream."] pub struct XzDecoder < R : Read > { inner : bufread :: XzDecoder < BufReader < R > > , }
};
}
