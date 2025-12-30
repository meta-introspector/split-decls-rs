// Generated macro for f32 (function)
macro_rules! Depcrate_number_completef32 {
() => {
// Module: crate::number::complete
// Provides: {"f32"}
// Dependencies: {}
# [doc = " Recognizes a 4 byte floating point number"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian f32 float,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian f32 float."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::f32;"] # [doc = ""] # [doc = " let be_f32 = |s| {"] # [doc = "   f32(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_f32(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(be_f32(&b\"abc\"[..]), Err(Err::Error((&b\"abc\"[..], ErrorKind::Eof))));"] # [doc = ""] # [doc = " let le_f32 = |s| {"] # [doc = "   f32(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_f32(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(le_f32(&b\"abc\"[..]), Err(Err::Error((&b\"abc\"[..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn f32 < I , E : ParseError < I > > (endian : crate :: number :: Endianness) -> fn (I) -> IResult < I , f32 , E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => be_f32 , crate :: number :: Endianness :: Little => le_f32 , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => be_f32 , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => le_f32 , } }
};
}
