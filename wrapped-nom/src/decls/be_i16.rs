macro_rules! deps {
    () => {
        Parser!();
        Input!();
        ErrorKind!();
        Needed!();
        Error!();
        ParseError!();
    };
}

macro_rules! be_i16 {
    () => {
        deps!();
        # [doc = " Recognizes a big endian signed 2 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i16;"] # [doc = ""] # [doc = " let mut parser = be_i16::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn be_i16 < I , E : ParseError < I > > () -> impl Parser < I , Output = i16 , Error = E > where I : Input < Item = u8 > , { be_u16 () . map (| x | x as i16) }
    };
}

be_i16!();