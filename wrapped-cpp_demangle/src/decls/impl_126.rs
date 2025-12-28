macro_rules! deps {
    () => {
        ParseContext!();
        Parse!();
        AbiTag!();
        Result!();
        IndexStr!();
        SourceName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl Parse for AbiTag { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (AbiTag , IndexStr < 'b >) > { try_begin_parse ! ("AbiTag" , ctx , input) ; let tail = consume (b"B" , input) ? ; let (source_name , tail) = SourceName :: parse (ctx , subs , tail) ? ; Ok ((AbiTag (source_name) , tail)) } }
    };
}

impl_126!();