macro_rules! deps {
    () => {
        ParseError!();
        Needed!();
        Error!();
        Input!();
        Parser!();
    };
}

macro_rules! le_i16 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian signed 2 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i16::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_i16 < I , E : ParseError < I > > () -> impl Parser < I , Output = i16 , Error = E > where I : Input < Item = u8 > , { le_u16 () . map (| x | x as i16) }
    };
}

le_i16!()