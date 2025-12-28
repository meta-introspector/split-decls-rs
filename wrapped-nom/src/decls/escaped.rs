macro_rules! deps {
    () => {
        AsChar!();
        Parser!();
        ParseError!();
        Input!();
        Needed!();
        Error!();
        IResult!();
        Escaped!();
        Offset!();
    };
}

macro_rules! escaped {
    () => {
        deps!();
        # [doc = " Matches a byte string with escaped characters."] # [doc = ""] # [doc = " * The first argument matches the normal characters (it must not accept the control character)"] # [doc = " * The second argument is the control character (like `\\` in most languages)"] # [doc = " * The third argument matches the escaped characters"] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " # use nom::character::complete::digit1;"] # [doc = " use nom::bytes::streaming::escaped;"] # [doc = " use nom::character::streaming::one_of;"] # [doc = ""] # [doc = " fn esc(s: &str) -> IResult<&str, &str> {"] # [doc = "   escaped(digit1, '\\\\', one_of(\"\\\"n\\\\\"))(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(esc(\"123;\"), Ok((\";\", \"123\")));"] # [doc = " assert_eq!(esc(\"12\\\\\\\"34;\"), Ok((\";\", \"12\\\\\\\"34\")));"] # [doc = " ```"] # [doc = ""] pub fn escaped < I , Error , F , G > (normal : F , control_char : char , escapable : G ,) -> impl Parser < I , Output = I , Error = Error > where I : Input + Clone + crate :: traits :: Offset , < I as Input > :: Item : crate :: traits :: AsChar , F : Parser < I , Error = Error > , G : Parser < I , Error = Error > , Error : ParseError < I > , { Escaped { normal , escapable , control_char , e : PhantomData , } }
    };
}

escaped!();