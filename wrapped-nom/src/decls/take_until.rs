macro_rules! deps {
    () => {
        Needed!();
        Streaming!();
        IResult!();
        Input!();
        FindSubstring!();
        ParseError!();
        Error!();
        TakeUntil!();
        Parser!();
    };
}

macro_rules! take_until {
    () => {
        deps!();
        # [doc = " Returns the input slice up to the first occurrence of the pattern."] # [doc = ""] # [doc = " It doesn't consume the pattern."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't"] # [doc = " contain the pattern or if the input is smaller than the pattern."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_until;"] # [doc = ""] # [doc = " fn until_eof(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_until(\"eof\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(until_eof(\"hello, worldeof\"), Ok((\"eof\", \"hello, world\")));"] # [doc = " assert_eq!(until_eof(\"hello, world\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(until_eof(\"hello, worldeo\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(until_eof(\"1eof2eof\"), Ok((\"eof2eof\", \"1\")));"] # [doc = " ```"] pub fn take_until < T , I , Error : ParseError < I > > (tag : T) -> impl Parser < I , Output = I , Error = Error > where I : Input + FindSubstring < T > , T : Clone , { TakeUntil { tag , e : PhantomData , } }
    };
}

take_until!();