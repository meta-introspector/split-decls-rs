macro_rules! deps {
    () => {
        ParseError!();
        Error!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        # [cfg (any (feature = "core-error" , feature = "std"))] impl Error for ParseError { # [allow (deprecated)] fn description (& self) -> & str { "parser error, see to_string() for details" } }
    };
}

impl_282!();