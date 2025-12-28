macro_rules! deps {
    () => {
        Endianness!();
        ParseError!();
        Error!();
        Needed!();
        Input!();
        Parser!();
        Either!();
    };
}

macro_rules! u64 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 8 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u64 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u64 integer."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::u64;"] # [doc = ""] # [doc = " let be_u64 = |s| {"] # [doc = "   u64::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u64(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(be_u64(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = ""] # [doc = " let le_u64 = |s| {"] # [doc = "   u64::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u64(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0706050403020100)));"] # [doc = " assert_eq!(le_u64(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn u64 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = u64 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_u64 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_u64 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_u64 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_u64 ()) , } }
    };
}

u64!()