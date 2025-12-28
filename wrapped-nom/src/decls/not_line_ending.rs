macro_rules! deps {
    () => {
        IResult!();
        Input!();
        Error!();
        ParseError!();
        Tag!();
        CompareResult!();
        ErrorKind!();
        Needed!();
        AsChar!();
        Streaming!();
        Err!();
        Compare!();
    };
}

macro_rules! not_line_ending {
    () => {
        deps!();
        # [doc = " Recognizes a string of any char except '\\r\\n' or '\\n'."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::streaming::not_line_ending;"] # [doc = " assert_eq!(not_line_ending::<_, (_, ErrorKind)>(\"ab\\r\\nc\"), Ok((\"\\r\\nc\", \"ab\")));"] # [doc = " assert_eq!(not_line_ending::<_, (_, ErrorKind)>(\"abc\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(not_line_ending::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(not_line_ending::<_, (_, ErrorKind)>(\"a\\rb\\nc\"), Err(Err::Error((\"a\\rb\\nc\", ErrorKind::Tag ))));"] # [doc = " assert_eq!(not_line_ending::<_, (_, ErrorKind)>(\"a\\rbc\"), Err(Err::Error((\"a\\rbc\", ErrorKind::Tag ))));"] # [doc = " ```"] pub fn not_line_ending < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , T : Compare < & 'static str > , < T as Input > :: Item : AsChar , { match input . position (| item | { let c = item . as_char () ; c == '\r' || c == '\n' }) { None => Err (Err :: Incomplete (Needed :: Unknown)) , Some (index) => { let mut it = input . take_from (index) . iter_elements () ; let nth = it . next () . unwrap () . as_char () ; if nth == '\r' { let sliced = input . take_from (index) ; let comp = sliced . compare ("\r\n") ; match comp { CompareResult :: Incomplete => Err (Err :: Incomplete (Needed :: Unknown)) , CompareResult :: Error => { let e : ErrorKind = ErrorKind :: Tag ; Err (Err :: Error (E :: from_error_kind (input , e))) } CompareResult :: Ok => Ok (input . take_split (index)) , } } else { Ok (input . take_split (index)) } } } }
    };
}

not_line_ending!()