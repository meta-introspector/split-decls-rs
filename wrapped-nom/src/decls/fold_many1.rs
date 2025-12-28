macro_rules! deps {
    () => {
        ErrorKind!();
        Parser!();
        IResult!();
        Input!();
        Needed!();
        ParseError!();
        FoldMany1!();
        Error!();
    };
}

macro_rules! fold_many1 {
    () => {
        deps!();
        # [doc = " Repeats the embedded parser, calling `g` to gather the results."] # [doc = ""] # [doc = " This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,"] # [doc = " see [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = " * `init` A function returning the initial value."] # [doc = " * `g` The function that combines a result of `f` with"] # [doc = "       the current accumulator."] # [doc = ""] # [doc = " *Note*: If the parser passed to `many1` accepts empty inputs"] # [doc = " (like `alpha0` or `digit0`), `many1` will return an error,"] # [doc = " to prevent going into an infinite loop."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::fold_many1;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   fold_many1("] # [doc = "     tag(\"abc\"),"] # [doc = "     Vec::new,"] # [doc = "     |mut acc: Vec<_>, item| {"] # [doc = "       acc.push(item);"] # [doc = "       acc"] # [doc = "     }"] # [doc = "   ).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"123123\"), Err(Err::Error(Error::new(\"123123\", ErrorKind::Many1))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Many1))));"] # [doc = " ```"] pub fn fold_many1 < I , E , F , G , H , R > (parser : F , init : H , g : G ,) -> impl Parser < I , Output = R , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , H : FnMut () -> R , E : ParseError < I > , { FoldMany1 { parser , g , init , r : PhantomData , } }
    };
}

fold_many1!()