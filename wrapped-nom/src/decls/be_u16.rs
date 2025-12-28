macro_rules! deps {
    () => {
        Needed!();
        ParseError!();
        Error!();
        Parser!();
        Input!();
    };
}

macro_rules! be_u16 {
    () => {
        deps!();
        # [doc = " Recognizes a big endian unsigned 2 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_u16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u16::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn be_u16 < I , E : ParseError < I > > () -> impl Parser < I , Output = u16 , Error = E > where I : Input < Item = u8 > , { be_uint (2) }
    };
}

be_u16!()