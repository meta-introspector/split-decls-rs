macro_rules! deps {
    () => {
        Error!();
        InvalidLengthError!();
        InvalidEncodingError!();
        Alphabet!();
    };
}

macro_rules! Encoding {
    () => {
        deps!();
        # [doc = " Base64 encoding trait."] # [doc = ""] # [doc = " This trait must be imported to make use of any Base64 alphabet defined"] # [doc = " in this crate."] # [doc = ""] # [doc = " The following encoding types impl this trait:"] # [doc = ""] # [doc = " - [`Base64`]: standard Base64 encoding with `=` padding."] # [doc = " - [`Base64Bcrypt`]: bcrypt Base64 encoding."] # [doc = " - [`Base64Crypt`]: `crypt(3)` Base64 encoding."] # [doc = " - [`Base64Unpadded`]: standard Base64 encoding *without* padding."] # [doc = " - [`Base64Url`]: URL-safe Base64 encoding with `=` padding."] # [doc = " - [`Base64UrlUnpadded`]: URL-safe Base64 encoding *without* padding."] pub trait Encoding : Alphabet { # [doc = " Decode a Base64 string into the provided destination buffer."] fn decode (src : impl AsRef < [u8] > , dst : & mut [u8]) -> Result < & [u8] , Error > ; # [doc = " Decode a Base64 string in-place."] # [doc = ""] # [doc = " NOTE: this method does not (yet) validate that padding is well-formed,"] # [doc = " if the given Base64 encoding is padded."] fn decode_in_place (buf : & mut [u8]) -> Result < & [u8] , InvalidEncodingError > ; # [doc = " Decode a Base64 string into a byte vector."] # [cfg (feature = "alloc")] fn decode_vec (input : & str) -> Result < Vec < u8 > , Error > ; # [doc = " Encode the input byte slice as Base64."] # [doc = ""] # [doc = " Writes the result into the provided destination slice, returning an"] # [doc = " ASCII-encoded Base64 string value."] fn encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , InvalidLengthError > ; # [doc = " Encode input byte slice into a [`String`] containing Base64."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `input` length is greater than `usize::MAX/4`."] # [cfg (feature = "alloc")] fn encode_string (input : & [u8]) -> String ; # [doc = " Get the length of Base64 produced by encoding the given bytes."] # [doc = ""] # [doc = " WARNING: this function will return `0` for lengths greater than `usize::MAX/4`!"] fn encoded_len (bytes : & [u8]) -> usize ; }
    };
}

Encoding!();