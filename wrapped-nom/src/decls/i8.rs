macro_rules! deps {
    () => {
        ParseError!();
        Needed!();
        Streaming!();
        Input!();
        Parser!();
        Error!();
    };
}

macro_rules! i8 {
    () => {
        deps!();
        # [doc = " Recognizes a signed 1 byte integer"] # [doc = ""] # [doc = " Note that endianness does not apply to 1 byte numbers."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::i8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   i8::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"\\x03abcefg\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn i8 < I , E : ParseError < I > > () -> impl Parser < I , Output = i8 , Error = E > where I : Input < Item = u8 > , { u8 () . map (| x | x as i8) }
    };
}

i8!()