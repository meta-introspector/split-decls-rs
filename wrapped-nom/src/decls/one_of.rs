macro_rules! deps {
    () => {
        ParseError!();
        AsChar!();
        Satisfy!();
        Input!();
        FindToken!();
        Parser!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! one_of {
    () => {
        deps!();
        # [doc = " Recognizes one of the provided characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind};"] # [doc = " # use nom::character::complete::one_of;"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"abc\")(\"b\"), Ok((\"\", 'b')));"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"a\")(\"bc\"), Err(Err::Error((\"bc\", ErrorKind::OneOf))));"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"a\")(\"\"), Err(Err::Error((\"\", ErrorKind::OneOf))));"] # [doc = " ```"] pub fn one_of < I , T , Error : ParseError < I > > (list : T) -> impl Parser < I , Output = char , Error = Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { Satisfy { predicate : move | c : char | list . find_token (c) , make_error : move | i | Error :: from_error_kind (i , ErrorKind :: OneOf) , } }
    };
}

one_of!();