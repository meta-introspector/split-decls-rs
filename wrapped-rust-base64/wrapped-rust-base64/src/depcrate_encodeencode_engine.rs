// Generated macro for encode_engine (function)
macro_rules! Depcrate_encodeencode_engine {
() => {
// Module: crate::encode
// Provides: {"encode_engine"}
// Dependencies: {}
# [doc = "Encode arbitrary octets as base64 using the provided `Engine` into a new `String`."] # [doc = ""] # [doc = " See [`Engine::encode`]."] # [allow (unused)] # [deprecated (since = "0.21.0" , note = "Use Engine::encode")] # [cfg (any (feature = "alloc" , test))] pub fn encode_engine < E : Engine , T : AsRef < [u8] > > (input : T , engine : & E) -> String { engine . encode (input) }
};
}
