macro_rules! deps {
    () => {
        Ast!();
        ParserI!();
    };
}

macro_rules! NestLimiter {
    () => {
        deps!();
        # [doc = " A type that traverses a fully parsed Ast and checks whether its depth"] # [doc = " exceeds the specified nesting limit. If it does, then an error is returned."] # [derive (Debug)] struct NestLimiter < 'p , 's , P > { # [doc = " The parser that is checking the nest limit."] p : & 'p ParserI < 's , P > , # [doc = " The current depth while walking an Ast."] depth : u32 , }
    };
}

NestLimiter!()