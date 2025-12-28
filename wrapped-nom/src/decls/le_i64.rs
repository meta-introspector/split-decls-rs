macro_rules! deps {
    () => {
        Needed!();
        Parser!();
        Error!();
        Input!();
        ParseError!();
    };
}

macro_rules! le_i64 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian signed 8 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i64::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcd\"[..]), Ok((&b\"abcd\"[..], 0x0706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn le_i64 < I , E : ParseError < I > > () -> impl Parser < I , Output = i64 , Error = E > where I : Input < Item = u8 > , { le_u64 () . map (| x | x as i64) }
    };
}

le_i64!();