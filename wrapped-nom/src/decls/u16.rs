macro_rules! deps {
    () => {
        Error!();
        Endianness!();
        Input!();
        ParseError!();
        Either!();
        Needed!();
        Parser!();
    };
}

macro_rules! u16 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 2 bytes integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u16 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u16 integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::u16;"] # [doc = ""] # [doc = " let be_u16 = |s| {"] # [doc = "   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0003)));"] # [doc = " assert_eq!(be_u16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = ""] # [doc = " let le_u16 = |s| {"] # [doc = "   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0300)));"] # [doc = " assert_eq!(le_u16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn u16 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = u16 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_u16 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_u16 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_u16 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_u16 ()) , } }
    };
}

u16!()