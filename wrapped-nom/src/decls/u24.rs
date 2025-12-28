macro_rules! deps {
    () => {
        Error!();
        Input!();
        Parser!();
        ParseError!();
        Endianness!();
        Either!();
        Needed!();
    };
}

macro_rules! u24 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 3 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u24 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u24 integer."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::u24;"] # [doc = ""] # [doc = " let be_u24 = |s| {"] # [doc = "   u24::<_,(_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x000305)));"] # [doc = " assert_eq!(be_u24(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = ""] # [doc = " let le_u24 = |s| {"] # [doc = "   u24::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(le_u24(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn u24 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = u32 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_u24 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_u24 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_u24 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_u24 ()) , } }
    };
}

u24!();