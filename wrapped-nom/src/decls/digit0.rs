macro_rules! deps {
    () => {
        IResult!();
        Needed!();
        Input!();
        ErrorKind!();
        Streaming!();
        ParseError!();
        AsChar!();
    };
}

macro_rules! digit0 {
    () => {
        deps!();
        # [doc = " Recognizes zero or more ASCII numerical characters: 0-9"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::digit0;"] # [doc = " assert_eq!(digit0::<_, (_, ErrorKind)>(\"21c\"), Ok((\"c\", \"21\")));"] # [doc = " assert_eq!(digit0::<_, (_, ErrorKind)>(\"a21c\"), Ok((\"a21c\", \"\")));"] # [doc = " assert_eq!(digit0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | ! item . is_dec_digit ()) }
    };
}

digit0!()