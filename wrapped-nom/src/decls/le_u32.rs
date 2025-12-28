macro_rules! deps {
    () => {
        Error!();
        Parser!();
        Input!();
        ParseError!();
        Needed!();
    };
}

macro_rules! le_u32 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian unsigned 4 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u32::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x03020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn le_u32 < I , E : ParseError < I > > () -> impl Parser < I , Output = u32 , Error = E > where I : Input < Item = u8 > , { le_uint (4) }
    };
}

le_u32!();