macro_rules! deps {
    () => {
        ErrorKind!();
        ParseError!();
        Input!();
        IResult!();
    };
}

macro_rules! rest {
    () => {
        deps!();
        # [doc = " Return the remaining input."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::error::ErrorKind;"] # [doc = " use nom::combinator::rest;"] # [doc = " assert_eq!(rest::<_,(_, ErrorKind)>(\"abc\"), Ok((\"\", \"abc\")));"] # [doc = " assert_eq!(rest::<_,(_, ErrorKind)>(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] # [inline] pub fn rest < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , { Ok (input . take_split (input . input_len ())) }
    };
}

rest!();