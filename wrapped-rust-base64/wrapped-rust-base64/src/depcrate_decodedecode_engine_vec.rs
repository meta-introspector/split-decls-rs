// Generated macro for decode_engine_vec (function)
macro_rules! Depcrate_decodedecode_engine_vec {
() => {
// Module: crate::decode
// Provides: {"decode_engine_vec"}
// Dependencies: {}
# [doc = " Decode from string reference as octets."] # [doc = ""] # [doc = " See [`Engine::decode_vec`]."] # [cfg (any (feature = "alloc" , test))] # [deprecated (since = "0.21.0" , note = "Use Engine::decode_vec")] pub fn decode_engine_vec < E : Engine , T : AsRef < [u8] > > (input : T , buffer : & mut Vec < u8 > , engine : & E ,) -> Result < () , DecodeError > { engine . decode_vec (input , buffer) }
};
}
