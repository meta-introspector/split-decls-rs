macro_rules! deps {
    () => {
        SimpleId!();
        DestructorName!();
        Parse!();
        OperatorName!();
        Name!();
        Result!();
        IndexStr!();
        ParseContext!();
        BaseUnresolvedName!();
        SubstitutionTable!();
        TemplateArgs!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl Parse for BaseUnresolvedName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (BaseUnresolvedName , IndexStr < 'b >) > { try_begin_parse ! ("BaseUnresolvedName" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (SimpleId :: parse (ctx , subs , input)) { return Ok ((BaseUnresolvedName :: Name (name) , tail)) ; } if let Ok (tail) = consume (b"on" , input) { let (opname , tail) = OperatorName :: parse (ctx , subs , tail) ? ; let (args , tail) = if let Ok ((args , tail)) = try_recurse ! (TemplateArgs :: parse (ctx , subs , tail)) { (Some (args) , tail) } else { (None , tail) } ; return Ok ((BaseUnresolvedName :: Operator (opname , args) , tail)) ; } let tail = consume (b"dn" , input) ? ; let (name , tail) = DestructorName :: parse (ctx , subs , tail) ? ; Ok ((BaseUnresolvedName :: Destructor (name) , tail)) } }
    };
}

impl_260!();