// Generated macro for impl_167 (impl)
macro_rules! Depcrate_engine_naiveimpl_167 {
() => {
// Module: crate::engine::naive
// Provides: {"impl_167"}
// Dependencies: {}
impl Naive { const ENCODE_INPUT_CHUNK_SIZE : usize = 3 ; const DECODE_INPUT_CHUNK_SIZE : usize = 4 ; pub const fn new (alphabet : & Alphabet , config : NaiveConfig) -> Self { Self { encode_table : encode_table (alphabet) , decode_table : decode_table (alphabet) , config , } } fn decode_byte_into_u32 (& self , offset : usize , byte : u8) -> Result < u32 , DecodeError > { let decoded = self . decode_table [byte as usize] ; if decoded == general_purpose :: INVALID_VALUE { return Err (DecodeError :: InvalidByte (offset , byte)) ; } Ok (decoded as u32) } }
};
}
