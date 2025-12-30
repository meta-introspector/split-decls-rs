// Generated macro for decode_secret (function)
macro_rules! Depcrate_base64decode_secret {
() => {
// Module: crate::base64
// Provides: {"decode_secret"}
// Dependencies: {}
# [doc = " Decode base64 `input`, writing the result into `output`."] # [doc = ""] # [doc = " `input` is treated as secret, so efforts are made to avoid"] # [doc = " leaking its value via side channels, such as timing,"] # [doc = " memory accesses, and execution trace."] # [doc = ""] # [doc = " The following is deemed non-secret information:"] # [doc = ""] # [doc = " - Appearance of whitespace in `input`"] # [doc = " - Erroneous characters in `input` (indeed, the first illegal"] # [doc = "   character is quoted in the error type)"] # [doc = " - The length of `input`"] # [doc = " - The length of `output`"] # [doc = ""] # [doc = " Returns the prefix of `output` that was written to."] pub (crate) fn decode_secret < 'a > (input : & [u8] , output : & 'a mut [u8]) -> Result < & 'a [u8] , Error > { decode (input , output , CodePoint :: decode_secret) }
};
}
