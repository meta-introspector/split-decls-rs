// Generated macro for decode_public (function)
macro_rules! Depcrate_base64decode_public {
() => {
// Module: crate::base64
// Provides: {"decode_public"}
// Dependencies: {}
# [doc = " Decode base64 `input`, writing the result into `output`."] # [doc = ""] # [doc = " `input` is treated as public information, so its value may"] # [doc = " be leaked via side channels."] # [doc = ""] # [doc = " Returns the prefix of `output` that was written to."] pub (crate) fn decode_public < 'a > (input : & [u8] , output : & 'a mut [u8]) -> Result < & 'a [u8] , Error > { decode (input , output , CodePoint :: decode_public) }
};
}
