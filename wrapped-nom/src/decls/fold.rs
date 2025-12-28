macro_rules! deps {
    () => {
        Parser!();
        Error!();
        ParseError!();
        Needed!();
        IResult!();
        NomRange!();
        Fold!();
        Input!();
    };
}

macro_rules! fold {
    () => {
        deps!();
        # [doc = " Applies a parser and accumulates the results using a given"] # [doc = " function and initial value."] # [doc = " Fails if the amount of time the embedded parser is run is not"] # [doc = " within the specified range."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `range` Constrains the number of iterations."] # [doc = "   * A range without an upper bound `a..` allows the parser to run until it fails."] # [doc = "   * A single `usize` value is equivalent to `value..=value`."] # [doc = "   * An empty range is invalid."] # [doc = " * `parse` The parser to apply."] # [doc = " * `init` A function returning the initial value."] # [doc = " * `fold` The function that combines a result of `f` with"] # [doc = "       the current accumulator."] # [doc = " ```rust"] # [doc = " # #[macro_use] extern crate nom;"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::multi::fold;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   fold("] # [doc = "     0..=2,"] # [doc = "     tag(\"abc\"),"] # [doc = "     Vec::new,"] # [doc = "     |mut acc: Vec<_>, item| {"] # [doc = "       acc.push(item);"] # [doc = "       acc"] # [doc = "     }"] # [doc = "   ).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"123123\"), Ok((\"123123\", vec![])));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", vec![])));"] # [doc = " assert_eq!(parser(\"abcabcabc\"), Ok((\"abc\", vec![\"abc\", \"abc\"])));"] # [doc = " ```"] pub fn fold < I , E , F , G , H , J , R > (range : J , parser : F , init : H , fold : G ,) -> impl Parser < I , Output = R , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , H : FnMut () -> R , E : ParseError < I > , J : NomRange < usize > , { Fold { parser , init , fold , range , } }
    };
}

fold!()