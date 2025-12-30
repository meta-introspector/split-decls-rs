// Generated macro for impl_124 (impl)
macro_rules! Depcrate_encodeimpl_124 {
() => {
// Module: crate::encode
// Provides: {"impl_124"}
// Dependencies: {}
impl Encoder { fn new () -> Encoder { Encoder { dst : vec ! [] } } fn finish (self) -> Vec < EncodeChunk > { self . dst } fn byte (& mut self , byte : u8) { if let Some (EncodeChunk :: EncodedBuf (buf)) = self . dst . last_mut () { buf . push (byte) ; } else { self . dst . push (EncodeChunk :: EncodedBuf (vec ! [byte])) ; } } fn extend_from_slice (& mut self , slice : & [u8]) { if let Some (EncodeChunk :: EncodedBuf (buf)) = self . dst . last_mut () { buf . extend_from_slice (slice) ; } else { self . dst . push (EncodeChunk :: EncodedBuf (slice . to_owned ())) ; } } }
};
}
