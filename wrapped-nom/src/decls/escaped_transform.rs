macro_rules! deps {
    () => {
        Parser!();
        IResult!();
        Error!();
        Offset!();
        ExtendInto!();
        AsChar!();
        ParseError!();
        EscapedTransform!();
        Needed!();
        Input!();
    };
}

macro_rules! escaped_transform {
    () => {
        deps!();
        # [doc = " Matches a byte string with escaped characters."] # [doc = ""] # [doc = " * The first argument matches the normal characters (it must not match the control character)"] # [doc = " * The second argument is the control character (like `\\` in most languages)"] # [doc = " * The third argument matches the escaped characters and transforms them"] # [doc = ""] # [doc = " As an example, the chain `abc\\tdef` could be `abc    def` (it also consumes the control character)"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " # use std::str::from_utf8;"] # [doc = " use nom::bytes::streaming::{escaped_transform, tag};"] # [doc = " use nom::character::streaming::alpha1;"] # [doc = " use nom::branch::alt;"] # [doc = " use nom::combinator::value;"] # [doc = ""] # [doc = " fn parser(input: &str) -> IResult<&str, String> {"] # [doc = "   escaped_transform("] # [doc = "     alpha1,"] # [doc = "     '\\\\',"] # [doc = "     alt(("] # [doc = "       value(\"\\\\\", tag(\"\\\\\")),"] # [doc = "       value(\"\\\"\", tag(\"\\\"\")),"] # [doc = "       value(\"\\n\", tag(\"n\")),"] # [doc = "     ))"] # [doc = "   )(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"ab\\\\\\\"cd\\\"\"), Ok((\"\\\"\", String::from(\"ab\\\"cd\"))));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn escaped_transform < I , Error , F , G , ExtendItem , Output > (normal : F , control_char : char , transform : G ,) -> impl Parser < I , Output = Output , Error = Error > where I : Clone + crate :: traits :: Offset + Input , I : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < F as Parser < I > > :: Output : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < G as Parser < I > > :: Output : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < I as Input > :: Item : crate :: traits :: AsChar , F : Parser < I , Error = Error > , G : Parser < I , Error = Error > , Error : ParseError < I > , { EscapedTransform { normal , control_char , transform , e : PhantomData , extend : PhantomData , o : PhantomData , } }
    };
}

escaped_transform!();