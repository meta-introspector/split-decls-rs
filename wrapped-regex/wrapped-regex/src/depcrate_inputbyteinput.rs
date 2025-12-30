// Generated macro for ByteInput (struct)
macro_rules! Depcrate_inputByteInput {
() => {
// Module: crate::input
// Provides: {"ByteInput"}
// Dependencies: {}
# [doc = " An input reader over bytes."] # [doc = ""] # [doc = " N.B. We represent the reader with a string for now, since that gives us"] # [doc = " easy access to necessary Unicode decoding (used for word boundary look"] # [doc = " ahead/look behind)."] # [derive (Clone , Copy , Debug)] pub struct ByteInput < 't > { text : & 't [u8] , only_utf8 : bool , }
};
}
