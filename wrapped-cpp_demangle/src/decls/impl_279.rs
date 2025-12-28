macro_rules! deps {
    () => {
        Result!();
        Parse!();
        SubstitutionTable!();
        ClosureTypeName!();
        ParseContext!();
        IndexStr!();
        LambdaSig!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl Parse for ClosureTypeName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ClosureTypeName , IndexStr < 'b >) > { try_begin_parse ! ("ClosureTypeName" , ctx , input) ; let tail = consume (b"Ul" , input) ? ; let (sig , tail) = LambdaSig :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let (num , tail) = if let Ok ((num , tail)) = parse_number (10 , false , tail) { (Some (num as _) , tail) } else { (None , tail) } ; let tail = consume (b"_" , tail) ? ; Ok ((ClosureTypeName (sig , num) , tail)) } }
    };
}

impl_279!()