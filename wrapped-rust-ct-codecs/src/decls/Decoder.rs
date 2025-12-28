macro_rules! deps {
    () => {
        Hex!();
        Error!();
        Base64!();
    };
}

macro_rules! Decoder {
    () => {
        deps!();
        # [doc = " Trait for decoding text representations back into binary data."] # [doc = ""] # [doc = " Implementors of this trait provide constant-time decoding operations"] # [doc = " for a specific encoding format (Base64, Hex, etc.)."] pub trait Decoder { # [doc = " Decodes text data back into its binary representation."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `bin` - Mutable buffer to store the decoded output"] # [doc = " * `encoded` - Text input data to decode"] # [doc = " * `ignore` - Optional set of characters to ignore during decoding (e.g., whitespace)"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(&[u8])` - A slice of the binary buffer containing the decoded data"] # [doc = " * `Err(Error::Overflow)` - If the output buffer is too small"] # [doc = " * `Err(Error::InvalidInput)` - If the input contains invalid characters"] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , encoded : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > ; # [doc = " Decodes text data and returns the result as a Vec<u8>."] # [doc = ""] # [doc = " This method is only available when the `std` feature is enabled."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `encoded` - Text input data to decode"] # [doc = " * `ignore` - Optional set of characters to ignore during decoding (e.g., whitespace)"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(Vec<u8>)` - A Vec containing the decoded binary data"] # [doc = " * `Err(Error::InvalidInput)` - If the input contains invalid characters"] # [cfg (feature = "std")] fn decode_to_vec < IN : AsRef < [u8] > > (encoded : IN , ignore : Option < & [u8] > ,) -> Result < Vec < u8 > , Error > { let mut bin = vec ! [0u8 ; encoded . as_ref () . len ()] ; let bin_len = Self :: decode (& mut bin , encoded , ignore) ? . len () ; bin . truncate (bin_len) ; Ok (bin) } }
    };
}

Decoder!();