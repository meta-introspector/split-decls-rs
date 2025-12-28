macro_rules! deps {
    () => {
        IndexStr!();
        Parse!();
        ParseContext!();
        Result!();
        SubstitutionTable!();
        UnnamedTypeName!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl Parse for UnnamedTypeName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnnamedTypeName , IndexStr < 'b >) > { try_begin_parse ! ("UnnamedTypeName" , ctx , input) ; let input = consume (b"Ut" , input) ? ; let (number , input) = match parse_number (10 , false , input) { Ok ((number , input)) => (Some (number as _) , input) , Err (_) => (None , input) , } ; let input = consume (b"_" , input) ? ; Ok ((UnnamedTypeName (number) , input)) } }
    };
}

impl_203!()