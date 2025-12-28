macro_rules! deps {
    () => {
        Header!();
        Parse!();
        Error!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        # [cfg (feature = "http1")] impl From < httparse :: Error > for Parse { fn from (err : httparse :: Error) -> Parse { match err { httparse :: Error :: HeaderName | httparse :: Error :: HeaderValue | httparse :: Error :: NewLine | httparse :: Error :: Token => Parse :: Header (Header :: Token) , httparse :: Error :: Status => Parse :: Status , httparse :: Error :: TooManyHeaders => Parse :: TooLarge , httparse :: Error :: Version => Parse :: Version , } } }
    };
}

impl_115!()