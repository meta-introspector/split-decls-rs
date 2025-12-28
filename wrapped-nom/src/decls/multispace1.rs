macro_rules! deps {
    () => {
        Streaming!();
        AsChar!();
        ErrorKind!();
        Input!();
        ParseError!();
        Needed!();
        IResult!();
    };
}

macro_rules! multispace1 {
    () => {
        deps!();
        # [doc = " Recognizes one or more spaces, tabs, carriage returns and line feeds."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::multispace1;"] # [doc = " assert_eq!(multispace1::<_, (_, ErrorKind)>(\" \\t\\n\\r21c\"), Ok((\"21c\", \" \\t\\n\\r\")));"] # [doc = " assert_eq!(multispace1::<_, (_, ErrorKind)>(\"H2\"), Err(Err::Error((\"H2\", ErrorKind::MultiSpace))));"] # [doc = " assert_eq!(multispace1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn multispace1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t' || c == '\r' || c == '\n') } , ErrorKind :: MultiSpace ,) }
    };
}

multispace1!()