macro_rules! deps {
    () => {
        ParseContext!();
        BareFunctionType!();
        Parse!();
        SubstitutionTable!();
        Result!();
        IndexStr!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl Parse for BareFunctionType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (BareFunctionType , IndexStr < 'b >) > { try_begin_parse ! ("BareFunctionType" , ctx , input) ; let (types , tail) = one_or_more :: < TypeHandle > (ctx , subs , input) ? ; Ok ((BareFunctionType (types) , tail)) } }
    };
}

impl_192!()