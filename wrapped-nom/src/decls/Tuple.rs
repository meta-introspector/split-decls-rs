macro_rules! deps {
    () => {
        Parser!();
        IResult!();
    };
}

macro_rules! Tuple {
    () => {
        deps!();
        # [doc = " Helper trait for the tuple combinator."] # [doc = ""] # [doc = " This trait is implemented for tuples of parsers of up to 21 elements."] # [deprecated (since = "8.0.0" , note = "`Parser` is directly implemented for tuples")] # [allow (deprecated)] pub trait Tuple < I , O , E > { # [doc = " Parses the input and returns a tuple of results of each parser."] fn parse_tuple (& mut self , input : I) -> IResult < I , O , E > ; }
    };
}

Tuple!();