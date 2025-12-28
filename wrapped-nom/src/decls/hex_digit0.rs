macro_rules! deps {
    () => {
        Input!();
        Needed!();
        Streaming!();
        ErrorKind!();
        ParseError!();
        AsChar!();
        IResult!();
    };
}

macro_rules! hex_digit0 {
    () => {
        deps!();
        # [doc = " Recognizes zero or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non hexadecimal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::hex_digit0;"] # [doc = " assert_eq!(hex_digit0::<_, (_, ErrorKind)>(\"21cZ\"), Ok((\"Z\", \"21c\")));"] # [doc = " assert_eq!(hex_digit0::<_, (_, ErrorKind)>(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(hex_digit0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn hex_digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | ! item . is_hex_digit ()) }
    };
}

hex_digit0!()