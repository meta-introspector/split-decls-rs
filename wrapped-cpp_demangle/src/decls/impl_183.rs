macro_rules! deps {
    () => {
        SubstitutionTable!();
        ParseContext!();
        ExceptionSpec!();
        Expression!();
        Parse!();
        Result!();
        IndexStr!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl Parse for ExceptionSpec { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ExceptionSpec , IndexStr < 'b >) > { try_begin_parse ! ("ExceptionSpec" , ctx , input) ; if let Ok (tail) = consume (b"Do" , input) { return Ok ((ExceptionSpec :: NoExcept , tail)) ; } let tail = consume (b"DO" , input) ? ; let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((ExceptionSpec :: Computed (expr) , tail)) } }
    };
}

impl_183!()