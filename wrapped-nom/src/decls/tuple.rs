macro_rules! deps {
    () => {
        ParseError!();
        IResult!();
        Tuple!();
        Parser!();
    };
}

macro_rules! tuple {
    () => {
        deps!();
        # [doc = "Applies a tuple of parsers one by one and returns their results as a tuple."] # [doc = "There is a maximum of 21 parsers"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind};"] # [doc = " use nom::sequence::tuple;"] # [doc = " use nom::character::complete::{alpha1, digit1};"] # [doc = " let mut parser = tuple((alpha1, digit1, alpha1));"] # [doc = ""] # [doc = " assert_eq!(parser(\"abc123def\"), Ok((\"\", (\"abc\", \"123\", \"def\"))));"] # [doc = " assert_eq!(parser(\"123def\"), Err(Err::Error((\"123def\", ErrorKind::Alpha))));"] # [doc = " ```"] # [deprecated (since = "8.0.0" , note = "`Parser` is directly implemented for tuples")] # [allow (deprecated)] pub fn tuple < I , O , E : ParseError < I > , List : Tuple < I , O , E > > (mut l : List ,) -> impl FnMut (I) -> IResult < I , O , E > { move | i : I | l . parse_tuple (i) }
    };
}

tuple!()