macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! ParseResult {
    () => {
        deps!();
        # [doc = " Same as `Result<T, ParseError>`."] pub type ParseResult < T > = Result < T , ParseError > ;
    };
}

ParseResult!()