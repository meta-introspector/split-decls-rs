macro_rules! deps {
    () => {
        Parser!();
        ParseError!();
        Needed!();
        Error!();
        Input!();
    };
}

macro_rules! u8 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 1 byte integer"] # [doc = ""] # [doc = " Note that endianness does not apply to 1 byte numbers."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::u8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   u8::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"\\x03abcefg\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn u8 < I , E : ParseError < I > > () -> impl Parser < I , Output = u8 , Error = E > where I : Input < Item = u8 > , { be_u8 () }
    };
}

u8!()