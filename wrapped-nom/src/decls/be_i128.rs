macro_rules! deps {
    () => {
        Input!();
        Error!();
        ParseError!();
        Parser!();
        ErrorKind!();
        Needed!();
    };
}

macro_rules! be_i128 {
    () => {
        deps!();
        # [doc = " Recognizes a big endian signed 16 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i128;"] # [doc = ""] # [doc = " let mut parser = be_i128::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x10\\x11\\x12\\x13\\x14\\x15abcd\"[..]), Ok((&b\"abcd\"[..], 0x00010203040506070809101112131415)));"] # [doc = " assert_eq!(parser.parse(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(15))));"] # [doc = " ```"] # [inline] pub fn be_i128 < I , E : ParseError < I > > () -> impl Parser < I , Output = i128 , Error = E > where I : Input < Item = u8 > , { be_u128 () . map (| x | x as i128) }
    };
}

be_i128!();