macro_rules! deps {
    () => {
        TimeZone!();
        LocalTimeType!();
        Error!();
        DateTime!();
        TransitionRule!();
        OutOfRange!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use Error :: * ; match self { DateTime (error) => write ! (f , "invalid date time: {error}") , FindLocalTimeType (error) => error . fmt (f) , LocalTimeType (error) => write ! (f , "invalid local time type: {error}") , InvalidSlice (error) => error . fmt (f) , InvalidTzString (error) => write ! (f , "invalid TZ string: {error}") , InvalidTzFile (error) => error . fmt (f) , Io (error) => error . fmt (f) , OutOfRange (error) => error . fmt (f) , ParseInt (error) => error . fmt (f) , ProjectDateTime (error) => error . fmt (f) , SystemTime (error) => error . fmt (f) , TransitionRule (error) => write ! (f , "invalid transition rule: {error}") , TimeZone (error) => write ! (f , "invalid time zone: {error}") , UnsupportedTzFile (error) => error . fmt (f) , UnsupportedTzString (error) => write ! (f , "unsupported TZ string: {error}") , Utf8 (error) => error . fmt (f) , } } }
    };
}

impl_641!()