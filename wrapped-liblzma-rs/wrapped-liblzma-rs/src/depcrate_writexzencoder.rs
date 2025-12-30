// Generated macro for XzEncoder (struct)
macro_rules! Depcrate_writeXzEncoder {
() => {
// Module: crate::write
// Provides: {"XzEncoder"}
// Dependencies: {}
# [doc = " A compression stream which will have uncompressed data written to it and"] # [doc = " will write compressed data to an output stream."] # [doc = " [XzEncoder] will no longer perform the finalization automatically in the next miner release, so you need to call [XzEncoder::finish] manually."] # [doc = " If you want to automate the finalization process, please use [XzEncoder::auto_finish]."] pub struct XzEncoder < W : Write > { data : Stream , obj : Option < W > , buf : Vec < u8 > , }
};
}
