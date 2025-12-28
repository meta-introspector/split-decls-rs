macro_rules! deps {
    () => {
        Needed!();
        Error!();
        AsChar!();
        Input!();
        FindToken!();
        ErrorKind!();
        Satisfy!();
        Parser!();
        ParseError!();
    };
}

macro_rules! none_of {
    () => {
        deps!();
        # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::character::streaming::none_of;"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"abc\")(\"z\"), Ok((\"\", 'z')));"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"ab\")(\"a\"), Err(Err::Error((\"a\", ErrorKind::NoneOf))));"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"a\")(\"\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " ```"] pub fn none_of < I , T , Error : ParseError < I > > (list : T) -> impl Parser < I , Output = char , Error = Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { Satisfy { predicate : move | c : char | ! list . find_token (c) , make_error : move | i | Error :: from_error_kind (i , ErrorKind :: NoneOf) , } }
    };
}

none_of!();