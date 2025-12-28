macro_rules! deps {
    () => {
        AsChar!();
        IResult!();
        Streaming!();
        Needed!();
        ParseError!();
        Input!();
        ErrorKind!();
    };
}

macro_rules! alpha0 {
    () => {
        deps!();
        # [doc = " Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non alphabetic character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::alpha0;"] # [doc = " assert_eq!(alpha0::<_, (_, ErrorKind)>(\"ab1c\"), Ok((\"1c\", \"ab\")));"] # [doc = " assert_eq!(alpha0::<_, (_, ErrorKind)>(\"1c\"), Ok((\"1c\", \"\")));"] # [doc = " assert_eq!(alpha0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn alpha0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | ! item . is_alpha ()) }
    };
}

alpha0!()