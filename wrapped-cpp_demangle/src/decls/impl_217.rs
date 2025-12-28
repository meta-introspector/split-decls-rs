macro_rules! deps {
    () => {
        Result!();
        ParseContext!();
        IndexStr!();
        Parse!();
        PointerToMemberType!();
        SubstitutionTable!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl Parse for PointerToMemberType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (PointerToMemberType , IndexStr < 'b >) > { try_begin_parse ! ("PointerToMemberType" , ctx , input) ; let tail = consume (b"M" , input) ? ; let (ty1 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let (ty2 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((PointerToMemberType (ty1 , ty2) , tail)) } }
    };
}

impl_217!()