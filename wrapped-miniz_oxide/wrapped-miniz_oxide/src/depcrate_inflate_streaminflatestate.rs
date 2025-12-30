// Generated macro for InflateState (struct)
macro_rules! Depcrate_inflate_streamInflateState {
() => {
// Module: crate::inflate::stream
// Provides: {"InflateState"}
// Dependencies: {}
# [doc = " A struct that compbines a decompressor with extra data for streaming decompression."] # [doc = ""] # [derive (Clone)] pub struct InflateState { # [doc = " Inner decompressor struct"] decomp : DecompressorOxide , # [doc = " Buffer of input bytes for matches."] # [doc = " TODO: Could probably do this a bit cleaner with some"] # [doc = " Cursor-like class."] # [doc = " We may also look into whether we need to keep a buffer here, or just one in the"] # [doc = " decompressor struct."] dict : [u8 ; TINFL_LZ_DICT_SIZE] , # [doc = " Where in the buffer are we currently at?"] dict_ofs : usize , # [doc = " How many bytes of data to be flushed is there currently in the buffer?"] dict_avail : usize , first_call : bool , has_flushed : bool , # [doc = " Whether the input data is wrapped in a zlib header and checksum."] # [doc = " TODO: This should be stored in the decompressor."] data_format : DataFormat , last_status : TINFLStatus , }
};
}
