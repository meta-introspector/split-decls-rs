macro_rules! deps {
    () => {
        Parse!();
        DataMemberPrefix!();
        SubstitutionTable!();
        SourceName!();
        IndexStr!();
        ParseContext!();
        Result!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl Parse for DataMemberPrefix { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (DataMemberPrefix , IndexStr < 'b >) > { try_begin_parse ! ("DataMemberPrefix" , ctx , input) ; let (name , tail) = SourceName :: parse (ctx , subs , input) ? ; let tail = consume (b"M" , tail) ? ; Ok ((DataMemberPrefix (name) , tail)) } }
    };
}

impl_289!();