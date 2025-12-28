macro_rules! deps {
    () => {
        ParseContext!();
        SubstitutionTable!();
        IndexStr!();
        AbiTags!();
        AbiTag!();
        Parse!();
        Result!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl Parse for AbiTags { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (AbiTags , IndexStr < 'b >) > { try_begin_parse ! ("AbiTags" , ctx , input) ; let (tags , tail) = zero_or_more :: < AbiTag > (ctx , subs , input) ? ; Ok ((AbiTags (tags) , tail)) } }
    };
}

impl_123!()