macro_rules! deps {
    () => {
        Parser!();
        ParseError!();
        Needed!();
        Input!();
        Error!();
    };
}

macro_rules! le_i32 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian signed 4 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i32::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x03020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn le_i32 < I , E : ParseError < I > > () -> impl Parser < I , Output = i32 , Error = E > where I : Input < Item = u8 > , { le_u32 () . map (| x | x as i32) }
    };
}

le_i32!()