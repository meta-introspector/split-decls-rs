macro_rules! deps {
    () => {
        Streaming!();
        Needed!();
        ParseError!();
        CompareResult!();
        IResult!();
        ErrorKind!();
        Err!();
        Input!();
        Compare!();
        Error!();
    };
}

macro_rules! crlf {
    () => {
        deps!();
        # [doc = " Recognizes the string \"\\r\\n\"."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::crlf;"] # [doc = " assert_eq!(crlf::<_, (_, ErrorKind)>(\"\\r\\nc\"), Ok((\"c\", \"\\r\\n\")));"] # [doc = " assert_eq!(crlf::<_, (_, ErrorKind)>(\"ab\\r\\nc\"), Err(Err::Error((\"ab\\r\\nc\", ErrorKind::CrLf))));"] # [doc = " assert_eq!(crlf::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] pub fn crlf < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , T : Compare < & 'static str > , { match input . compare ("\r\n") { CompareResult :: Ok => Ok (input . take_split (2)) , CompareResult :: Incomplete => Err (Err :: Incomplete (Needed :: new (2))) , CompareResult :: Error => { let e : ErrorKind = ErrorKind :: CrLf ; Err (Err :: Error (E :: from_error_kind (input , e))) } } }
    };
}

crlf!()