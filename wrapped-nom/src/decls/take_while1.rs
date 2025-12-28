macro_rules! deps {
    () => {
        Input!();
        ErrorKind!();
        SplitPosition1!();
        Error!();
        Parser!();
        Needed!();
        ParseError!();
        Streaming!();
        IResult!();
    };
}

macro_rules! take_while1 {
    () => {
        deps!();
        # [doc = " Returns the longest (at least 1) input slice that matches the predicate."] # [doc = ""] # [doc = " The parser will return the longest slice that matches the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = ""] # [doc = " It will return an `Err(Err::Error((_, ErrorKind::TakeWhile1)))` if the pattern wasn't met."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(1))` or if the pattern reaches the end of the input."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_while1;"] # [doc = " use nom::AsChar;"] # [doc = ""] # [doc = " fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   take_while1(AsChar::is_alpha)(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(alpha(b\"latin123\"), Ok((&b\"123\"[..], &b\"latin\"[..])));"] # [doc = " assert_eq!(alpha(b\"latin\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(alpha(b\"12345\"), Err(Err::Error(Error::new(&b\"12345\"[..], ErrorKind::TakeWhile1))));"] # [doc = " ```"] pub fn take_while1 < F , I , Error : ParseError < I > > (cond : F) -> impl Parser < I , Output = I , Error = Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { SplitPosition1 { e : ErrorKind :: TakeWhile1 , predicate : move | c | ! cond (c) , error : PhantomData , } }
    };
}

take_while1!();