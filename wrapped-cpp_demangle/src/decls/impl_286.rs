macro_rules! deps {
    () => {
        Parse!();
        ParseContext!();
        SubstitutionTable!();
        Result!();
        LambdaSig!();
        IndexStr!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl Parse for LambdaSig { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (LambdaSig , IndexStr < 'b >) > { try_begin_parse ! ("LambdaSig" , ctx , input) ; let (types , tail) = if let Ok (tail) = consume (b"v" , input) { (vec ! [] , tail) } else { one_or_more :: < TypeHandle > (ctx , subs , input) ? } ; Ok ((LambdaSig (types) , tail)) } }
    };
}

impl_286!();