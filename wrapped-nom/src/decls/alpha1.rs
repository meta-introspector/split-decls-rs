macro_rules! deps {
    () => {
        Input!();
        Streaming!();
        Needed!();
        ErrorKind!();
        ParseError!();
        AsChar!();
        IResult!();
    };
}

macro_rules! alpha1 {
    () => {
        deps!();
        # [doc = " Recognizes one or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non alphabetic character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::alpha1;"] # [doc = " assert_eq!(alpha1::<_, (_, ErrorKind)>(\"aB1c\"), Ok((\"1c\", \"aB\")));"] # [doc = " assert_eq!(alpha1::<_, (_, ErrorKind)>(\"1c\"), Err(Err::Error((\"1c\", ErrorKind::Alpha))));"] # [doc = " assert_eq!(alpha1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn alpha1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_alpha () , ErrorKind :: Alpha) }
    };
}

alpha1!()