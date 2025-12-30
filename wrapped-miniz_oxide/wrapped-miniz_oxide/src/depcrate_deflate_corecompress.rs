// Generated macro for compress (function)
macro_rules! Depcrate_deflate_corecompress {
() => {
// Module: crate::deflate::core
// Provides: {"compress"}
// Dependencies: {}
# [doc = " Main compression function. Tries to compress as much as possible from `in_buf` and"] # [doc = " puts compressed output into `out_buf`."] # [doc = ""] # [doc = " The value of `flush` determines if the compressor should attempt to flush all output"] # [doc = " and alternatively try to finish the stream."] # [doc = ""] # [doc = " Use [`TDEFLFlush::Finish`] on the final call to signal that the stream is finishing."] # [doc = ""] # [doc = " Note that this function does not keep track of whether a flush marker has been output, so"] # [doc = " if called using [`TDEFLFlush::Sync`], the caller needs to ensure there is enough space in the"] # [doc = " output buffer if they want to avoid repeated flush markers."] # [doc = " See #105 for details."] # [doc = ""] # [doc = " # Returns"] # [doc = " Returns a tuple containing the current status of the compressor, the current position"] # [doc = " in the input buffer and the current position in the output buffer."] pub fn compress (d : & mut CompressorOxide , in_buf : & [u8] , out_buf : & mut [u8] , flush : TDEFLFlush ,) -> (TDEFLStatus , usize , usize) { compress_inner (d , & mut CallbackOxide :: new_callback_buf (in_buf , out_buf) , flush ,) }
};
}
