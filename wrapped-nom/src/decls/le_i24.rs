macro_rules! deps {
    () => {
        Needed!();
        ParseError!();
        Parser!();
        Error!();
        Input!();
    };
}

macro_rules! le_i24 {
    () => {
        deps!();
        # [doc = " Recognizes a little endian signed 3 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i24::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn le_i24 < I , E : ParseError < I > > () -> impl Parser < I , Output = i32 , Error = E > where I : Input < Item = u8 > , { le_u24 () . map (| x | { if x & 0x80_00_00 != 0 { (x | 0xff_00_00_00) as i32 } else { x as i32 } }) }
    };
}

le_i24!();