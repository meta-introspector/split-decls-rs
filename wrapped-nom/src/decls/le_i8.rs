macro_rules! deps {
    () => {
        Error!();
        Parser!();
        Input!();
        ErrorKind!();
        Needed!();
        ParseError!();
    };
}

macro_rules! le_i8 {
    () => {
        deps!();
        # [doc = " Recognizes a signed 1 byte integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i8;"] # [doc = ""] # [doc = " let mut parser = le_i8::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_i8 < I , E : ParseError < I > > () -> impl Parser < I , Output = i8 , Error = E > where I : Input < Item = u8 > , { le_u8 () . map (| x | x as i8) }
    };
}

le_i8!()