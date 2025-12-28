macro_rules! deps {
    () => {
        ErrorKind!();
        Needed!();
        Streaming!();
        IResult!();
        ParseError!();
        Input!();
        AsChar!();
    };
}

macro_rules! alphanumeric0 {
    () => {
        deps!();
        # [doc = " Recognizes zero or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non alphanumerical character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::alphanumeric0;"] # [doc = " assert_eq!(alphanumeric0::<_, (_, ErrorKind)>(\"21cZ%1\"), Ok((\"%1\", \"21cZ\")));"] # [doc = " assert_eq!(alphanumeric0::<_, (_, ErrorKind)>(\"&Z21c\"), Ok((\"&Z21c\", \"\")));"] # [doc = " assert_eq!(alphanumeric0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn alphanumeric0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | ! item . is_alphanum ()) }
    };
}

alphanumeric0!()