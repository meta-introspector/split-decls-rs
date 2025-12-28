macro_rules! deps {
    () => {
        ParseError!();
        Streaming!();
        Input!();
        Needed!();
        Parser!();
        Error!();
    };
}

macro_rules! be_f64 {
    () => {
        deps!();
        # [doc = " Recognizes a big endian 8 bytes floating point number."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_f64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_f64::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn be_f64 < I , E : ParseError < I > > () -> impl Parser < I , Output = f64 , Error = E > where I : Input < Item = u8 > , { be_u64 () . map (f64 :: from_bits) }
    };
}

be_f64!()