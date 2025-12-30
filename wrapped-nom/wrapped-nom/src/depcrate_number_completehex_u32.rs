// Generated macro for hex_u32 (function)
macro_rules! Depcrate_number_completehex_u32 {
() => {
// Module: crate::number::complete
// Provides: {"hex_u32"}
// Dependencies: {}
# [doc = " Recognizes a hex-encoded integer."] # [doc = ""] # [doc = " *Complete version*: Will parse until the end of input if it has less than 8 bytes."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::hex_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   hex_u32(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"01AE\"[..]), Ok((&b\"\"[..], 0x01AE)));"] # [doc = " assert_eq!(parser(&b\"abc\"[..]), Ok((&b\"\"[..], 0x0ABC)));"] # [doc = " assert_eq!(parser(&b\"ggg\"[..]), Err(Err::Error((&b\"ggg\"[..], ErrorKind::IsA))));"] # [doc = " ```"] # [inline] pub fn hex_u32 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input , < I as Input > :: Item : AsChar , I : AsBytes , { let e : ErrorKind = ErrorKind :: IsA ; let (i , o) = input . split_at_position1_complete (| c | { let c = c . as_char () ; ! "0123456789abcdefABCDEF" . contains (c) } , e ,) ? ; let (remaining , parsed) = if o . input_len () <= 8 { (i , o) } else { input . take_split (8) } ; let res = parsed . as_bytes () . iter () . rev () . enumerate () . map (| (k , & v) | { let digit = v as char ; digit . to_digit (16) . unwrap_or (0) << (k * 4) }) . sum () ; Ok ((remaining , res)) }
};
}
