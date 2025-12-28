macro_rules! deps {
    () => {
        ErrorKind!();
        Input!();
        ParseError!();
        Needed!();
        Streaming!();
        Error!();
        IResult!();
        AsChar!();
    };
}

macro_rules! newline {
    () => {
        deps!();
        # [doc = " Matches a newline character '\\\\n'."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::newline;"] # [doc = " assert_eq!(newline::<_, (_, ErrorKind)>(\"\\nc\"), Ok((\"c\", '\\n')));"] # [doc = " assert_eq!(newline::<_, (_, ErrorKind)>(\"\\r\\nc\"), Err(Err::Error((\"\\r\\nc\", ErrorKind::Char))));"] # [doc = " assert_eq!(newline::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn newline < I , Error : ParseError < I > > (input : I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , { char ('\n') (input) }
    };
}

newline!()