// Generated macro for add_padding (function)
macro_rules! Depcrate_encodeadd_padding {
() => {
// Module: crate::encode
// Provides: {"add_padding"}
// Dependencies: {}
# [doc = " Write padding characters."] # [doc = " `unpadded_output_len` is the size of the unpadded but base64 encoded data."] # [doc = " `output` is the slice where padding should be written, of length at least 2."] # [doc = ""] # [doc = " Returns the number of padding bytes written."] pub (crate) fn add_padding (unpadded_output_len : usize , output : & mut [u8]) -> usize { let pad_bytes = (4 - (unpadded_output_len % 4)) % 4 ; # [allow (clippy :: needless_range_loop)] for i in 0 .. pad_bytes { output [i] = PAD_BYTE ; } pad_bytes }
};
}
