macro_rules! deps {
    () => {
        FromHexError!();
    };
}

macro_rules! decode_in_slice {
    () => {
        deps!();
        # [doc = " Decode a hex string into itself."] # [doc = ""] # [doc = " Both, upper and lower case characters are valid in the input string and can"] # [doc = " even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut bytes: [u8; 8] = *b\"6b697769\";"] # [doc = " assert_eq!(hex::decode_in_slice(&mut bytes), Ok(()));"] # [doc = " assert_eq!(&bytes[0..bytes.len() / 2], b\"kiwi\");"] # [doc = " ```"] pub fn decode_in_slice (in_out : & mut [u8]) -> Result < () , FromHexError > { if in_out . len () % 2 != 0 { return Err (FromHexError :: OddLength) ; } for i in 0 .. (in_out . len () / 2) { let byte = val (& in_out [2 * i .. 2 * i + 2] , 2 * i) ? ; in_out [i] = byte ; } Ok (()) }
    };
}

decode_in_slice!()