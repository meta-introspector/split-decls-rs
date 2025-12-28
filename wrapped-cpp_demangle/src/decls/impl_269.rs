macro_rules! deps {
    () => {
        Initializer!();
        Result!();
        Parse!();
        Expression!();
        ParseContext!();
        SubstitutionTable!();
        IndexStr!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl Parse for Initializer { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Initializer , IndexStr < 'b >) > { try_begin_parse ! ("Initializer" , ctx , input) ; let tail = consume (b"pi" , input) ? ; let (exprs , tail) = zero_or_more :: < Expression > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((Initializer (exprs) , tail)) } }
    };
}

impl_269!()