// Generated macro for decode_lossy (function)
macro_rules! Depcrate_utf8decode_lossy {
() => {
// Module: crate::utf8
// Provides: {"decode_lossy"}
// Dependencies: {}
# [doc = " Like `decode`, but automatically converts the `None` case to the"] # [doc = " replacement codepoint."] pub (crate) fn decode_lossy < B : AsRef < [u8] > > (slice : B) -> (char , usize) { match decode (slice) { (Some (ch) , size) => (ch , size) , (None , size) => ('\u{FFFD}' , size) , } }
};
}
