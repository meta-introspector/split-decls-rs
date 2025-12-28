macro_rules! deps {
    () => {
        Endianness!();
        Parser!();
        ParseError!();
        Input!();
        Error!();
        Either!();
        Needed!();
    };
}

macro_rules! u128 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 16 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u128 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u128 integer."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::u128;"] # [doc = ""] # [doc = " let be_u128 = |s| {"] # [doc = "   u128::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u128(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x00010203040506070001020304050607)));"] # [doc = " assert_eq!(be_u128(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(15))));"] # [doc = ""] # [doc = " let le_u128 = |s| {"] # [doc = "   u128::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u128(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x07060504030201000706050403020100)));"] # [doc = " assert_eq!(le_u128(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(15))));"] # [doc = " ```"] # [inline] pub fn u128 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = u128 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_u128 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_u128 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_u128 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_u128 ()) , } }
    };
}

u128!();