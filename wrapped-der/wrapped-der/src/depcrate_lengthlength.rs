// Generated macro for Length (struct)
macro_rules! Depcrate_lengthLength {
() => {
// Module: crate::length
// Provides: {"Length"}
// Dependencies: {}
# [doc = " ASN.1-encoded length."] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use der::{Decode, Length, SliceReader};"] # [doc = ""] # [doc = " let mut reader = SliceReader::new(&[0x82, 0xAA, 0xBB]).unwrap();"] # [doc = " let length = Length::decode(&mut reader).expect(\"valid length\");"] # [doc = ""] # [doc = " assert_eq!(length, Length::new(0xAABB));"] # [doc = " ```"] # [doc = ""] # [doc = " 5-byte lengths are supported:"] # [doc = " ```"] # [doc = " use der::{Encode, Length};"] # [doc = " let length = Length::new(0x10000000);"] # [doc = ""] # [doc = " assert_eq!(length.encoded_len(), Ok(Length::new(5)));"] # [doc = " ```"] # [doc = ""] # [doc = " Invalid lengths produce an error:"] # [doc = " ```"] # [doc = " use der::{Decode, Length, SliceReader};"] # [doc = ""] # [doc = " let mut reader = SliceReader::new(&[0x81, 0x7F]).unwrap();"] # [doc = ""] # [doc = " Length::decode(&mut reader).expect_err(\"non-canonical length should be rejected\");"] # [doc = " ```"] # [derive (Copy , Clone , Default , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct Length { # [doc = " Inner length as a `u32`. Note that the decoder and encoder also support a maximum length"] # [doc = " of 32-bits."] inner : u32 , # [doc = " Flag bit which specifies whether the length was indeterminate when decoding ASN.1 BER."] # [doc = ""] # [doc = " This should always be false when working with DER."] # [cfg (feature = "ber")] indefinite : bool , }
};
}
