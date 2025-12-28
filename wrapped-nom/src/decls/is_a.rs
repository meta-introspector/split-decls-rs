macro_rules! deps {
    () => {
        ErrorKind!();
        IResult!();
        ParseError!();
        FindToken!();
        Error!();
        Needed!();
        Input!();
        Parser!();
        SplitPosition1!();
    };
}

macro_rules! is_a {
    () => {
        deps!();
        # [doc = " Returns the longest input slice (at least 1) that matches the pattern."] # [doc = ""] # [doc = " The parser will return the longest slice consisting of the characters in provided in the"] # [doc = " combinator's argument."] # [doc = ""] # [doc = " It will return a `Err(Err::Error((_, ErrorKind::IsA)))` if the pattern wasn't met."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::complete::is_a;"] # [doc = ""] # [doc = " fn hex(s: &str) -> IResult<&str, &str> {"] # [doc = "   is_a(\"1234567890ABCDEF\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(hex(\"123 and voila\"), Ok((\" and voila\", \"123\")));"] # [doc = " assert_eq!(hex(\"DEADBEEF and others\"), Ok((\" and others\", \"DEADBEEF\")));"] # [doc = " assert_eq!(hex(\"BADBABEsomething\"), Ok((\"something\", \"BADBABE\")));"] # [doc = " assert_eq!(hex(\"D15EA5E\"), Ok((\"\", \"D15EA5E\")));"] # [doc = " assert_eq!(hex(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::IsA))));"] # [doc = " ```"] pub fn is_a < T , I , Error : ParseError < I > > (arr : T) -> impl Parser < I , Output = I , Error = Error > where I : Input , T : FindToken < < I as Input > :: Item > , { SplitPosition1 { e : ErrorKind :: IsA , predicate : move | c | ! arr . find_token (c) , error : PhantomData , } }
    };
}

is_a!();