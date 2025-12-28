macro_rules! ParseResult {
    () => {
        # [doc = " Type alias to simplify specifying the return value of chained closures."] pub type ParseResult < S > = Result < S , S > ;
    };
}

ParseResult!();