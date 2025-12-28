macro_rules! deps {
    () => {
        Error!();
        Streaming!();
        Needed!();
        Endianness!();
        Either!();
        Parser!();
        Input!();
        ParseError!();
    };
}

macro_rules! f64 {
    () => {
        deps!();
        # [doc = " Recognizes an 8 byte floating point number"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian f64 float,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian f64 float."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::f64;"] # [doc = ""] # [doc = " let be_f64 = |s| {"] # [doc = "   f64::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_f64(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(be_f64(&b\"abc\"[..]), Err(Err::Incomplete(Needed::new(5))));"] # [doc = ""] # [doc = " let le_f64 = |s| {"] # [doc = "   f64::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(le_f64(&b\"abc\"[..]), Err(Err::Incomplete(Needed::new(5))));"] # [doc = " ```"] # [inline] pub fn f64 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = f64 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_f64 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_f64 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_f64 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_f64 ()) , } }
    };
}

f64!()