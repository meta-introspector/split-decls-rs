// Generated macro for impl_12 (impl)
macro_rules! Depcrate_chunked_encoderimpl_12 {
() => {
// Module: crate::chunked_encoder
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (any (feature = "alloc" , test))] impl < 'a > Sink for StringSink < 'a > { type Error = () ; fn write_encoded_bytes (& mut self , s : & [u8]) -> Result < () , Self :: Error > { self . string . push_str (str :: from_utf8 (s) . unwrap ()) ; Ok (()) } }
};
}
