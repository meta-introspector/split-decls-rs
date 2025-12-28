macro_rules! deps {
    () => {
        SubstitutionTable!();
        ParseContext!();
        BaseUnresolvedName!();
        UnresolvedQualifierLevel!();
        Result!();
        UnresolvedName!();
        Name!();
        IndexStr!();
        Parse!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl Parse for UnresolvedName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnresolvedName , IndexStr < 'b >) > { try_begin_parse ! ("UnresolvedName" , ctx , input) ; if let Ok (tail) = consume (b"gs" , input) { if let Ok ((name , tail)) = try_recurse ! (BaseUnresolvedName :: parse (ctx , subs , tail)) { return Ok ((UnresolvedName :: Global (name) , tail)) ; } let tail = consume (b"sr" , tail) ? ; let (levels , tail) = one_or_more :: < UnresolvedQualifierLevel > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let (name , tail) = BaseUnresolvedName :: parse (ctx , subs , tail) ? ; return Ok ((UnresolvedName :: GlobalNested2 (levels , name) , tail)) ; } if let Ok ((name , tail)) = try_recurse ! (BaseUnresolvedName :: parse (ctx , subs , input)) { return Ok ((UnresolvedName :: Name (name) , tail)) ; } let tail = consume (b"sr" , input) ? ; if tail . peek () == Some (b'N') { let tail = consume (b"N" , tail) . unwrap () ; let (ty , tail) = UnresolvedTypeHandle :: parse (ctx , subs , tail) ? ; let (levels , tail) = one_or_more :: < UnresolvedQualifierLevel > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let (name , tail) = BaseUnresolvedName :: parse (ctx , subs , tail) ? ; return Ok ((UnresolvedName :: Nested1 (ty , levels , name) , tail)) ; } if let Ok ((ty , tail)) = try_recurse ! (UnresolvedTypeHandle :: parse (ctx , subs , tail)) { let (name , tail) = BaseUnresolvedName :: parse (ctx , subs , tail) ? ; return Ok ((UnresolvedName :: Nested1 (ty , vec ! [] , name) , tail)) ; } let (levels , tail) = one_or_more :: < UnresolvedQualifierLevel > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let (name , tail) = BaseUnresolvedName :: parse (ctx , subs , tail) ? ; Ok ((UnresolvedName :: Nested2 (levels , name) , tail)) } }
    };
}

impl_247!()