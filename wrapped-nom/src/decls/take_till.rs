macro_rules! deps {
    () => {
        IResult!();
        Input!();
        Error!();
        SplitPosition!();
        Needed!();
        ParseError!();
        Parser!();
    };
}

macro_rules! take_till {
    () => {
        deps!();
        # [doc = " Returns the longest input slice (if any) till a predicate is met."] # [doc = ""] # [doc = " The parser will return the longest slice till the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::complete::take_till;"] # [doc = ""] # [doc = " fn till_colon(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_till(|c| c == ':')(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(till_colon(\"latin:123\"), Ok((\":123\", \"latin\")));"] # [doc = " assert_eq!(till_colon(\":empty matched\"), Ok((\":empty matched\", \"\"))); //allowed"] # [doc = " assert_eq!(till_colon(\"12345\"), Ok((\"\", \"12345\")));"] # [doc = " assert_eq!(till_colon(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] # [allow (clippy :: redundant_closure)] pub fn take_till < F , I , Error : ParseError < I > > (cond : F) -> impl Parser < I , Output = I , Error = Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { SplitPosition { predicate : cond , error : PhantomData , } }
    };
}

take_till!();