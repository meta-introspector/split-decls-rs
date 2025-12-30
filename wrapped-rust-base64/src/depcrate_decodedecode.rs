// Generated macro for decode (function)
macro_rules! Depcrate_decodedecode {
() => {
// Module: crate::decode
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode base64 using the [`STANDARD` engine](STANDARD)."] # [doc = ""] # [doc = " See [`Engine::decode`]."] # [deprecated (since = "0.21.0" , note = "Use Engine::decode")] # [cfg (any (feature = "alloc" , test))] pub fn decode < T : AsRef < [u8] > > (input : T) -> Result < Vec < u8 > , DecodeError > { STANDARD . decode (input) }
};
}
