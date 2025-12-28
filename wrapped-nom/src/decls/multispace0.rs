macro_rules! deps {
    () => {
        IResult!();
        Needed!();
        Error!();
        AsChar!();
        ParseError!();
        Streaming!();
        ErrorKind!();
        Parser!();
        Input!();
        MultiSpace0!();
    };
}

macro_rules! multispace0 {
    () => {
        deps!();
        # [doc = " Recognizes zero or more spaces, tabs, carriage returns and line feeds."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::multispace0;"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\" \\t\\n\\r21c\"), Ok((\"21c\", \" \\t\\n\\r\")));"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn multispace0 < T , E : ParseError < T > > () -> impl Parser < T , Output = T , Error = E > where T : Input , < T as Input > :: Item : AsChar , { MultiSpace0 { e : PhantomData } }
    };
}

multispace0!()