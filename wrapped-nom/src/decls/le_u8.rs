macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        ParseError!();
        Needed!();
        Parser!();
        Input!();
    };
}

macro_rules! le_u8 {
    () => {
        deps!();
        # [doc = " Recognizes an unsigned 1 byte integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_u8;"] # [doc = ""] # [doc = " let mut parser = le_u8::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_u8 < I , E : ParseError < I > > () -> impl Parser < I , Output = u8 , Error = E > where I : Input < Item = u8 > , { le_uint (1) }
    };
}

le_u8!();