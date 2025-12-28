macro_rules! deps {
    () => {
        SubstitutionTable!();
        Parse!();
        Result!();
        IndexStr!();
        ParseContext!();
        UnresolvedQualifierLevel!();
        SimpleId!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl Parse for UnresolvedQualifierLevel { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnresolvedQualifierLevel , IndexStr < 'b >) > { try_begin_parse ! ("UnresolvedQualifierLevel" , ctx , input) ; let (id , tail) = SimpleId :: parse (ctx , subs , input) ? ; Ok ((UnresolvedQualifierLevel (id) , tail)) } }
    };
}

impl_254!()