// Generated macro for decode_engine_slice (function)
macro_rules! Depcrate_decodedecode_engine_slice {
() => {
// Module: crate::decode
// Provides: {"decode_engine_slice"}
// Dependencies: {}
# [doc = " Decode the input into the provided output slice."] # [doc = ""] # [doc = " See [`Engine::decode_slice`]."] # [deprecated (since = "0.21.0" , note = "Use Engine::decode_slice")] pub fn decode_engine_slice < E : Engine , T : AsRef < [u8] > > (input : T , output : & mut [u8] , engine : & E ,) -> Result < usize , DecodeSliceError > { engine . decode_slice (input , output) }
};
}
