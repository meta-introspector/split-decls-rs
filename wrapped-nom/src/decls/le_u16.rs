macro_rules! deps {
    () => {
        Parser!();
        Error!();
        ParseError!();
        Input!();
        Needed!();
    };
}

macro_rules! le_u16 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian unsigned 2 bytes integer."] # [doc = ""] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_u16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u16::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_u16 < I , E : ParseError < I > > () -> impl Parser < I , Output = u16 , Error = E > where I : Input < Item = u8 > , { le_uint (2) }
    };
}

le_u16!()