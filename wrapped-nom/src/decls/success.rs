macro_rules! deps {
    () => {
        Error!();
        Parser!();
        Success!();
        IResult!();
        ParseError!();
    };
}

macro_rules! success {
    () => {
        deps!();
        # [doc = " a parser which always succeeds with given value without consuming any input."] # [doc = ""] # [doc = " It can be used for example as the last alternative in `alt` to"] # [doc = " specify the default case."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::branch::alt;"] # [doc = " use nom::combinator::{success, value};"] # [doc = " use nom::character::complete::char;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = success::<_,_,(_,ErrorKind)>(10);"] # [doc = " assert_eq!(parser.parse(\"xyz\"), Ok((\"xyz\", 10)));"] # [doc = ""] # [doc = " let mut sign = alt((value(-1, char('-')), value(1, char('+')), success::<_,_,(_,ErrorKind)>(1)));"] # [doc = " assert_eq!(sign.parse(\"+10\"), Ok((\"10\", 1)));"] # [doc = " assert_eq!(sign.parse(\"-10\"), Ok((\"10\", -1)));"] # [doc = " assert_eq!(sign.parse(\"10\"), Ok((\"10\", 1)));"] # [doc = " # }"] # [doc = " ```"] pub fn success < I , O : Clone , E : ParseError < I > > (val : O) -> impl Parser < I , Output = O , Error = E > { Success { val , e : PhantomData , } }
    };
}

success!();