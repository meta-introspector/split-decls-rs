// Generated macro for many_till (function)
macro_rules! Depcrate_multimany_till {
() => {
// Module: crate::multi
// Provides: {"many_till"}
// Dependencies: {}
# [doc = " Applies the parser `f` until the parser `g` produces a result."] # [doc = ""] # [doc = " Returns a tuple of the results of `f` in a `Vec` and the result of `g`."] # [doc = ""] # [doc = " `f` keeps going so long as `g` produces [`Err::Error`]. To instead chain an error up, see [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::many_till;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {"] # [doc = "   many_till(tag(\"abc\"), tag(\"end\")).parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabcend\"), Ok((\"\", (vec![\"abc\", \"abc\"], \"end\"))));"] # [doc = " assert_eq!(parser(\"abc123end\"), Err(Err::Error(Error::new(\"123end\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"123123end\"), Err(Err::Error(Error::new(\"123123end\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"abcendefg\"), Ok((\"efg\", (vec![\"abc\"], \"end\"))));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn many_till < I , E , F , G > (f : F , g : G ,) -> impl Parser < I , Output = (Vec < < F as Parser < I > > :: Output > , < G as Parser < I > > :: Output) , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { ManyTill { f , g , e : PhantomData , } }
};
}
