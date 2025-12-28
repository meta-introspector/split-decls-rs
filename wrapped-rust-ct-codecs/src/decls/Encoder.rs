macro_rules! deps {
    () => {
        Hex!();
        Error!();
        Base64!();
    };
}

macro_rules! Encoder {
    () => {
        deps!();
        # [doc = " Trait for encoding binary data into text representations."] # [doc = ""] # [doc = " Implementors of this trait provide constant-time encoding operations"] # [doc = " for a specific encoding format (Base64, Hex, etc.)."] pub trait Encoder { # [doc = " Calculates the length of the encoded output for a given binary input length."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `bin_len` - The length of the binary input in bytes"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(usize)` - The required length for the encoded output"] # [doc = " * `Err(Error::Overflow)` - If the calculation would overflow"] fn encoded_len (bin_len : usize) -> Result < usize , Error > ; # [doc = " Encodes binary data into a text representation."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `encoded` - Mutable buffer to store the encoded output"] # [doc = " * `bin` - Binary input data to encode"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(&[u8])` - A slice of the encoded buffer containing the encoded data"] # [doc = " * `Err(Error::Overflow)` - If the output buffer is too small"] fn encode < IN : AsRef < [u8] > > (encoded : & mut [u8] , bin : IN) -> Result < & [u8] , Error > ; # [doc = " Encodes binary data and returns the result as a string slice."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `encoded` - Mutable buffer to store the encoded output"] # [doc = " * `bin` - Binary input data to encode"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(&str)` - A string slice containing the encoded data"] # [doc = " * `Err(Error::Overflow)` - If the output buffer is too small"] fn encode_to_str < IN : AsRef < [u8] > > (encoded : & mut [u8] , bin : IN) -> Result < & str , Error > { Ok (core :: str :: from_utf8 (Self :: encode (encoded , bin) ?) . unwrap ()) } # [doc = " Encodes binary data and returns the result as a String."] # [doc = ""] # [doc = " This method is only available when the `std` feature is enabled."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `bin` - Binary input data to encode"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(String)` - A String containing the encoded data"] # [doc = " * `Err(Error::Overflow)` - If the calculation would overflow"] # [cfg (feature = "std")] fn encode_to_string < IN : AsRef < [u8] > > (bin : IN) -> Result < String , Error > { let mut encoded = vec ! [0u8 ; Self :: encoded_len (bin . as_ref () . len ()) ?] ; let encoded_len = Self :: encode (& mut encoded , bin) ? . len () ; encoded . truncate (encoded_len) ; Ok (String :: from_utf8 (encoded) . unwrap ()) } }
    };
}

Encoder!();