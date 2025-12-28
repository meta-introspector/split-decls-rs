macro_rules! deps {
    () => {
        Parse!();
        Result!();
        ParseContext!();
        Name!();
        SubstitutionTable!();
        SimpleId!();
        DestructorName!();
        IndexStr!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl Parse for DestructorName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (DestructorName , IndexStr < 'b >) > { try_begin_parse ! ("DestructorName" , ctx , input) ; if let Ok ((ty , tail)) = try_recurse ! (UnresolvedTypeHandle :: parse (ctx , subs , input)) { return Ok ((DestructorName :: Unresolved (ty) , tail)) ; } let (name , tail) = SimpleId :: parse (ctx , subs , input) ? ; Ok ((DestructorName :: Name (name) , tail)) } }
    };
}

impl_263!();