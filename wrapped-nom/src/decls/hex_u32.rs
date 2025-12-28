macro_rules! deps {
    () => {
        IResult!();
        AsBytes!();
        AsChar!();
        Streaming!();
        ParseError!();
        Needed!();
        Input!();
        ErrorKind!();
    };
}

macro_rules! hex_u32 {
    () => {
        deps!();
        # [doc = " Recognizes a hex-encoded integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::hex_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   hex_u32(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"01AE;\"[..]), Ok((&b\";\"[..], 0x01AE)));"] # [doc = " assert_eq!(parser(&b\"abc\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(parser(&b\"ggg\"[..]), Err(Err::Error((&b\"ggg\"[..], ErrorKind::IsA))));"] # [doc = " ```"] # [inline] pub fn hex_u32 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input + AsBytes , < I as Input > :: Item : AsChar , { let e : ErrorKind = ErrorKind :: IsA ; let (i , o) = input . split_at_position1 (| c | { let c = c . as_char () ; ! "0123456789abcdefABCDEF" . contains (c) } , e ,) ? ; let (remaining , parsed) = if o . input_len () <= 8 { (i , o) } else { input . take_split (8) } ; let res = parsed . as_bytes () . iter () . rev () . enumerate () . map (| (k , & v) | { let digit = v as char ; digit . to_digit (16) . unwrap_or (0) << (k * 4) }) . sum () ; Ok ((remaining , res)) }
    };
}

hex_u32!();