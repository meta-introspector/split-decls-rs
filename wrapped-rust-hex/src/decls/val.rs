macro_rules! deps {
    () => {
        FromHexError!();
    };
}

macro_rules! val {
    () => {
        deps!();
        # [inline] fn val (bytes : & [u8] , idx : usize) -> Result < u8 , FromHexError > { let upper = DECODE_TABLE [bytes [0] as usize] ; let lower = DECODE_TABLE [bytes [1] as usize] ; if upper == u8 :: MAX { return Err (FromHexError :: InvalidHexCharacter { c : bytes [0] as char , index : idx , }) ; } if lower == u8 :: MAX { return Err (FromHexError :: InvalidHexCharacter { c : bytes [1] as char , index : idx + 1 , }) ; } Ok ((upper << 4) | lower) }
    };
}

val!();