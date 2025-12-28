macro_rules! deps {
    () => {
        Alphabet!();
        DecodeError!();
        NaiveConfig!();
        Naive!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Naive { const ENCODE_INPUT_CHUNK_SIZE : usize = 3 ; const DECODE_INPUT_CHUNK_SIZE : usize = 4 ; pub const fn new (alphabet : & Alphabet , config : NaiveConfig) -> Self { Self { encode_table : encode_table (alphabet) , decode_table : decode_table (alphabet) , config , } } fn decode_byte_into_u32 (& self , offset : usize , byte : u8) -> Result < u32 , DecodeError > { let decoded = self . decode_table [byte as usize] ; if decoded == general_purpose :: INVALID_VALUE { return Err (DecodeError :: InvalidByte (offset , byte)) ; } Ok (decoded as u32) } }
    };
}

impl_127!()