// Generated macro for decode (function)
macro_rules! Depcrate_util_utf8decode {
() => {
// Module: crate::util::utf8
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decodes the next UTF-8 encoded codepoint from the given byte slice."] # [doc = ""] # [doc = " If no valid encoding of a codepoint exists at the beginning of the"] # [doc = " given byte slice, then a 1-3 byte slice is returned (which is guaranteed"] # [doc = " to be a prefix of `bytes`). That byte slice corresponds either to a single"] # [doc = " invalid byte, or to a prefix of a valid UTF-8 encoding of a Unicode scalar"] # [doc = " value (but which ultimately did not lead to a valid encoding)."] # [doc = ""] # [doc = " This returns `None` if and only if `bytes` is empty."] # [doc = ""] # [doc = " This never panics."] # [doc = ""] # [doc = " *WARNING*: This is not designed for performance. If you're looking for a"] # [doc = " fast UTF-8 decoder, this is not it. If you feel like you need one in this"] # [doc = " crate, then please file an issue and discuss your use case."] pub (crate) fn decode (bytes : & [u8]) -> Option < Result < char , & [u8] > > { crate :: shared :: util :: utf8 :: decode (bytes) }
};
}
