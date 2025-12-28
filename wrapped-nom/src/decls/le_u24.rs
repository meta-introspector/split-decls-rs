macro_rules! deps {
    () => {
        Parser!();
        Error!();
        Input!();
        ParseError!();
        Needed!();
    };
}

macro_rules! le_u24 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian unsigned 3 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_u24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u24::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn le_u24 < I , E : ParseError < I > > () -> impl Parser < I , Output = u32 , Error = E > where I : Input < Item = u8 > , { le_uint (3) }
    };
}

le_u24!()