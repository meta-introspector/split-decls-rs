// Generated macro for encode (function)
macro_rules! Depcrate_encodeencode {
() => {
// Module: crate::encode
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Encode arbitrary octets as base64 using the [`STANDARD` engine](STANDARD)."] # [doc = ""] # [doc = " See [`Engine::encode`]."] # [allow (unused)] # [deprecated (since = "0.21.0" , note = "Use Engine::encode")] # [cfg (any (feature = "alloc" , test))] pub fn encode < T : AsRef < [u8] > > (input : T) -> String { STANDARD . encode (input) }
};
}
