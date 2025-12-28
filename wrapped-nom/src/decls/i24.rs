macro_rules! deps {
    () => {
        Input!();
        Streaming!();
        Endianness!();
        ParseError!();
        Needed!();
        Either!();
        Error!();
        Parser!();
    };
}

macro_rules! i24 {
    () => {
        deps!();
        # [doc = " Recognizes a signed 3 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian i24 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian i24 integer."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::i24;"] # [doc = ""] # [doc = " let be_i24 = |s| {"] # [doc = "   i24::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_i24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x000305)));"] # [doc = " assert_eq!(be_i24(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = ""] # [doc = " let le_i24 = |s| {"] # [doc = "   i24::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_i24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(le_i24(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn i24 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = i32 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_i24 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_i24 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_i24 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_i24 ()) , } }
    };
}

i24!();