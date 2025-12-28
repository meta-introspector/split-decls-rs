macro_rules! deps {
    () => {
        Streaming!();
        Input!();
        CompareResult!();
        ParseError!();
        Compare!();
        ErrorKind!();
        Err!();
        Error!();
        IResult!();
        Needed!();
    };
}

macro_rules! line_ending {
    () => {
        deps!();
        # [doc = " Recognizes an end of line (both '\\n' and '\\r\\n')."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::line_ending;"] # [doc = " assert_eq!(line_ending::<_, (_, ErrorKind)>(\"\\r\\nc\"), Ok((\"c\", \"\\r\\n\")));"] # [doc = " assert_eq!(line_ending::<_, (_, ErrorKind)>(\"ab\\r\\nc\"), Err(Err::Error((\"ab\\r\\nc\", ErrorKind::CrLf))));"] # [doc = " assert_eq!(line_ending::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn line_ending < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , T : Compare < & 'static str > , { match input . compare ("\n") { CompareResult :: Ok => Ok (input . take_split (1)) , CompareResult :: Incomplete => Err (Err :: Incomplete (Needed :: new (1))) , CompareResult :: Error => { match input . compare ("\r\n") { CompareResult :: Ok => Ok (input . take_split (2)) , CompareResult :: Incomplete => Err (Err :: Incomplete (Needed :: new (2))) , CompareResult :: Error => Err (Err :: Error (E :: from_error_kind (input , ErrorKind :: CrLf))) , } } } }
    };
}

line_ending!();