macro_rules! deps {
    () => {
        ParseError!();
        Parser!();
        FoldManyMN!();
        Error!();
        Input!();
        Needed!();
        IResult!();
    };
}

macro_rules! fold_many_m_n {
    () => {
        deps!();
        # [doc = " Repeats the embedded parser `m..=n` times, calling `g` to gather the results"] # [doc = ""] # [doc = " This stops before `n` when the parser returns [`Err::Error`]. To instead chain an error up, see"] # [doc = " [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `m` The minimum number of iterations."] # [doc = " * `n` The maximum number of iterations."] # [doc = " * `f` The parser to apply."] # [doc = " * `init` A function returning the initial value."] # [doc = " * `g` The function that combines a result of `f` with"] # [doc = "       the current accumulator."] # [doc = ""] # [doc = " *Note*: If the parser passed to `many1` accepts empty inputs"] # [doc = " (like `alpha0` or `digit0`), `many1` will return an error,"] # [doc = " to prevent going into an infinite loop."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::multi::fold_many_m_n;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   fold_many_m_n("] # [doc = "     0,"] # [doc = "     2,"] # [doc = "     tag(\"abc\"),"] # [doc = "     Vec::new,"] # [doc = "     |mut acc: Vec<_>, item| {"] # [doc = "       acc.push(item);"] # [doc = "       acc"] # [doc = "     }"] # [doc = "   ).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"123123\"), Ok((\"123123\", vec![])));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", vec![])));"] # [doc = " assert_eq!(parser(\"abcabcabc\"), Ok((\"abc\", vec![\"abc\", \"abc\"])));"] # [doc = " ```"] pub fn fold_many_m_n < I , E , F , G , H , R > (min : usize , max : usize , parser : F , init : H , g : G ,) -> impl Parser < I , Output = R , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , H : FnMut () -> R , E : ParseError < I > , { FoldManyMN { parser , g , init , min , max , r : PhantomData , } }
    };
}

fold_many_m_n!();