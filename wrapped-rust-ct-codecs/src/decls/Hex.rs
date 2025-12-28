macro_rules! deps {
    () => {
        Encoder!();
        Decoder!();
    };
}

macro_rules! Hex {
    () => {
        deps!();
        # [doc = " Hexadecimal encoder and decoder implementation."] # [doc = ""] # [doc = " Provides constant-time encoding and decoding of binary data to and from"] # [doc = " hexadecimal representation. The implementation uses only lowercase"] # [doc = " hexadecimal characters (0-9, a-f) for encoding."] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " All operations run in constant time relative to input length, making"] # [doc = " this implementation suitable for handling sensitive cryptographic data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ct_codecs::{Hex, Encoder, Decoder};"] # [doc = ""] # [doc = " fn example() -> Result<(), ct_codecs::Error> {"] # [doc = "     let data = b\"Hello\";"] # [doc = ""] # [doc = "     // Encode binary data to hex"] # [doc = "     let encoded = Hex::encode_to_string(data)?;"] # [doc = "     assert_eq!(encoded, \"48656c6c6f\");"] # [doc = ""] # [doc = "     // Decode hex back to binary"] # [doc = "     let decoded = Hex::decode_to_vec(&encoded, None)?;"] # [doc = "     assert_eq!(decoded, data);"] # [doc = ""] # [doc = "     // Working with preallocated buffers (useful for no_std)"] # [doc = "     let mut hex_buf = [0u8; 10];"] # [doc = "     let hex = Hex::encode(&mut hex_buf, data)?;"] # [doc = "     assert_eq!(hex, b\"48656c6c6f\");"] # [doc = ""] # [doc = "     let mut bin_buf = [0u8; 5];"] # [doc = "     let bin = Hex::decode(&mut bin_buf, hex, None)?;"] # [doc = "     assert_eq!(bin, data);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " # example().unwrap();"] # [doc = " ```"] pub struct Hex ;
    };
}

Hex!()