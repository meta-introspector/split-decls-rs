macro_rules! deps {
    () => {
        Parser!();
        ParseError!();
        Error!();
        IResult!();
        Fail!();
    };
}

macro_rules! fail {
    () => {
        deps!();
        # [doc = " A parser which always fails."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::fail;"] # [doc = ""] # [doc = " let s = \"string\";"] # [doc = " assert_eq!(fail::<_, &str, _>().parse(s), Err(Err::Error((s, ErrorKind::Fail))));"] # [doc = " ```"] pub fn fail < I , O , E : ParseError < I > > () -> impl Parser < I , Output = O , Error = E > { Fail { o : PhantomData , e : PhantomData , } }
    };
}

fail!();