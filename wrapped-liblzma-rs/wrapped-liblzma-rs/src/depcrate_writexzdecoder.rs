// Generated macro for XzDecoder (struct)
macro_rules! Depcrate_writeXzDecoder {
() => {
// Module: crate::write
// Provides: {"XzDecoder"}
// Dependencies: {}
# [doc = " A compression stream which will have compressed data written to it and"] # [doc = " will write uncompressed data to an output stream."] # [doc = " [XzDecoder] will no longer perform the finalization automatically in the next miner release, so you need to call [XzDecoder::finish] manually."] # [doc = " If you want to automate the finalization process, please use [XzDecoder::auto_finish]."] pub struct XzDecoder < W : Write > { data : Stream , obj : Option < W > , buf : Vec < u8 > , }
};
}
