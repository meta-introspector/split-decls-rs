// Generated macro for Header (struct)
macro_rules! Depcrate_headerHeader {
() => {
// Module: crate::header
// Provides: {"Header"}
// Dependencies: {}
# [doc = " ASN.1 DER headers: tag + length component of TLV-encoded values"] # [doc = ""] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use der::{Decode, Header, Length, Reader, SliceReader, Tag};"] # [doc = ""] # [doc = " let mut reader = SliceReader::new(&[0x04, 0x02, 0x31, 0x32]).unwrap();"] # [doc = " let header = Header::decode(&mut reader).expect(\"valid header\");"] # [doc = ""] # [doc = " assert_eq!(header, Header::new(Tag::OctetString, Length::new(2)));"] # [doc = ""] # [doc = " assert_eq!(reader.read_slice(2u8.into()).unwrap(), b\"12\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use der::{Encode, Header, Length, Tag};"] # [doc = " let header = Header::new(Tag::Sequence, Length::new(256));"] # [doc = ""] # [doc = " // Header of 256-byte SEQUENCE is 4-byte long"] # [doc = " assert_eq!(header.encoded_len(), Ok(Length::new(4)));"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Header { # [doc = " Tag representing the type of the encoded value"] tag : Tag , # [doc = " Length of the encoded value"] length : Length , # [doc = " True if value is constructed, rather than primitive"] constructed : bool , }
};
}
