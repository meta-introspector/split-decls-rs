macro_rules! deps {
    () => {
        ErrorKind!();
        Ast!();
        Span!();
        Parser!();
        ParserI!();
        Result!();
        NestLimiter!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'p , 's , P : Borrow < Parser > > NestLimiter < 'p , 's , P > { fn new (p : & 'p ParserI < 's , P >) -> NestLimiter < 'p , 's , P > { NestLimiter { p , depth : 0 } } # [inline (never)] fn check (self , ast : & Ast) -> Result < () > { ast :: visit (ast , self) } fn increment_depth (& mut self , span : & Span) -> Result < () > { let new = self . depth . checked_add (1) . ok_or_else (| | { self . p . error (span . clone () , ast :: ErrorKind :: NestLimitExceeded (u32 :: MAX) ,) }) ? ; let limit = self . p . parser () . nest_limit ; if new > limit { return Err (self . p . error (span . clone () , ast :: ErrorKind :: NestLimitExceeded (limit) ,)) ; } self . depth = new ; Ok (()) } fn decrement_depth (& mut self) { self . depth = self . depth . checked_sub (1) . unwrap () ; } }
    };
}

impl_18!();