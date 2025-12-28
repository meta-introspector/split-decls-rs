macro_rules! deps {
    () => {
        ErrorKind!();
        ParseError!();
        Input!();
        Parser!();
        Needed!();
        Error!();
    };
}

macro_rules! be_i64 {
    () => {
        deps!();
        # [doc = " Recognizes a big endian signed 8 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i64;"] # [doc = ""] # [doc = " let mut parser = be_i64::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(parser.parse(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn be_i64 < I , E : ParseError < I > > () -> impl Parser < I , Output = i64 , Error = E > where I : Input < Item = u8 > , { be_u64 () . map (| x | x as i64) }
    };
}

be_i64!();