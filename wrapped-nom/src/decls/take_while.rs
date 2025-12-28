macro_rules! deps {
    () => {
        ParseError!();
        IResult!();
        Parser!();
        SplitPosition!();
        Error!();
        Input!();
        Needed!();
    };
}

macro_rules! take_while {
    () => {
        deps!();
        # [doc = " Returns the longest input slice (if any) that matches the predicate."] # [doc = ""] # [doc = " The parser will return the longest slice that matches the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::complete::take_while;"] # [doc = " use nom::AsChar;"] # [doc = ""] # [doc = " fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   take_while(AsChar::is_alpha)(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(alpha(b\"latin123\"), Ok((&b\"123\"[..], &b\"latin\"[..])));"] # [doc = " assert_eq!(alpha(b\"12345\"), Ok((&b\"12345\"[..], &b\"\"[..])));"] # [doc = " assert_eq!(alpha(b\"latin\"), Ok((&b\"\"[..], &b\"latin\"[..])));"] # [doc = " assert_eq!(alpha(b\"\"), Ok((&b\"\"[..], &b\"\"[..])));"] # [doc = " ```"] pub fn take_while < F , I , Error : ParseError < I > > (cond : F) -> impl Parser < I , Output = I , Error = Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { SplitPosition { predicate : move | c | ! cond (c) , error : PhantomData , } }
    };
}

take_while!()