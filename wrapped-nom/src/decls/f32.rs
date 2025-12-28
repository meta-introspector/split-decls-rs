macro_rules! deps {
    () => {
        Streaming!();
        Error!();
        Parser!();
        Needed!();
        ParseError!();
        Endianness!();
        Either!();
        Input!();
    };
}

macro_rules! f32 {
    () => {
        deps!();
        # [doc = " Recognizes a 4 byte floating point number"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian f32 float,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian f32 float."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::f32;"] # [doc = ""] # [doc = " let be_f32 = |s| {"] # [doc = "   f32::<_, (_, ErrorKind)>(nom::number::Endianness::Big).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_f32(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(be_f32(&b\"abc\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = ""] # [doc = " let le_f32 = |s| {"] # [doc = "   f32::<_, (_, ErrorKind)>(nom::number::Endianness::Little).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_f32(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(le_f32(&b\"abc\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn f32 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Parser < I , Output = f32 , Error = E > where I : Input < Item = u8 > , { match endian { crate :: number :: Endianness :: Big => Either :: Left (be_f32 ()) , crate :: number :: Endianness :: Little => Either :: Right (le_f32 ()) , # [cfg (target_endian = "big")] crate :: number :: Endianness :: Native => Either :: Left (be_f32 ()) , # [cfg (target_endian = "little")] crate :: number :: Endianness :: Native => Either :: Right (le_f32 ()) , } }
    };
}

f32!();