macro_rules! deps {
    () => {
        Parser!();
        Error!();
        Streaming!();
        Either!();
        ParseError!();
        Endianness!();
        Input!();
        Needed!();
    };
}

macro_rules! i16 {
    () => {
        deps!();
        # [doc = " Recognizes a signed 2 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian i16 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian i16 integer."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::i16;"] # [doc = ""] # [doc = " let be_i16 = |s| {"] # [doc = "   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_i16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0003)));"] # [doc = " assert_eq!(be_i16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = ""] # [doc = " let le_i16 = |s| {"] # [doc = "   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_i16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0300)));"] # [doc = " assert_eq!(le_i16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn i16 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = i16 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_i16 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_i16 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_i16 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_i16 ()) , } }
    };
}

i16!();