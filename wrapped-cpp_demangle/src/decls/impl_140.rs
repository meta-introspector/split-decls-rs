macro_rules! deps {
    () => {
        Result!();
        IndexStr!();
        Parse!();
        ParseContext!();
        OperatorName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Parse for OperatorName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (OperatorName , IndexStr < 'b >) > { OperatorName :: parse_internal (ctx , subs , input , false) } }
    };
}

impl_140!()