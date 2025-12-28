macro_rules! deps {
    () => {
        IResult!();
        AsChar!();
        ErrorKind!();
        Needed!();
        ParseError!();
        Streaming!();
        Input!();
    };
}

macro_rules! oct_digit1 {
    () => {
        deps!();
        # [doc = " Recognizes one or more octal characters: 0-7"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non octal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::oct_digit1;"] # [doc = " assert_eq!(oct_digit1::<_, (_, ErrorKind)>(\"21cZ\"), Ok((\"cZ\", \"21\")));"] # [doc = " assert_eq!(oct_digit1::<_, (_, ErrorKind)>(\"H2\"), Err(Err::Error((\"H2\", ErrorKind::OctDigit))));"] # [doc = " assert_eq!(oct_digit1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn oct_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_oct_digit () , ErrorKind :: OctDigit) }
    };
}

oct_digit1!()