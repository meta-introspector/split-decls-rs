macro_rules! deps {
    () => {
        FromHex!();
        FromHexError!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        # [doc = " Decodes a hex string into raw bytes."] # [doc = ""] # [doc = " Both, upper and lower case characters are valid in the input string and can"] # [doc = " even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!("] # [doc = "     hex::decode(\"48656c6c6f20776f726c6421\"),"] # [doc = "     Ok(\"Hello world!\".to_owned().into_bytes())"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!(hex::decode(\"123\"), Err(hex::FromHexError::OddLength));"] # [doc = " assert!(hex::decode(\"foo\").is_err());"] # [doc = " ```"] # [cfg (feature = "alloc")] pub fn decode < T : AsRef < [u8] > > (data : T) -> Result < Vec < u8 > , FromHexError > { FromHex :: from_hex (data) }
    };
}

decode!()