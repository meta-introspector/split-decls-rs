macro_rules! deps {
    () => {
        ParseContext!();
        SubstitutionTable!();
        Result!();
        TemplateParam!();
        IndexStr!();
        Decltype!();
        Substitution!();
        Parse!();
        Substitutable!();
        TemplateArgs!();
        UnresolvedType!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl Parse for UnresolvedTypeHandle { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnresolvedTypeHandle , IndexStr < 'b >) > { try_begin_parse ! ("UnresolvedTypeHandle" , ctx , input) ; if let Ok ((param , tail)) = try_recurse ! (TemplateParam :: parse (ctx , subs , input)) { let (args , tail) = if let Ok ((args , tail)) = try_recurse ! (TemplateArgs :: parse (ctx , subs , tail)) { (Some (args) , tail) } else { (None , tail) } ; let ty = UnresolvedType :: Template (param , args) ; let ty = Substitutable :: UnresolvedType (ty) ; let idx = subs . insert (ty) ; let handle = UnresolvedTypeHandle :: BackReference (idx) ; return Ok ((handle , tail)) ; } if let Ok ((decltype , tail)) = try_recurse ! (Decltype :: parse (ctx , subs , input)) { let ty = UnresolvedType :: Decltype (decltype) ; let ty = Substitutable :: UnresolvedType (ty) ; let idx = subs . insert (ty) ; let handle = UnresolvedTypeHandle :: BackReference (idx) ; return Ok ((handle , tail)) ; } let (sub , tail) = Substitution :: parse (ctx , subs , input) ? ; match sub { Substitution :: WellKnown (component) => { Ok ((UnresolvedTypeHandle :: WellKnown (component) , tail)) } Substitution :: BackReference (idx) => { Ok ((UnresolvedTypeHandle :: BackReference (idx) , tail)) } } } }
    };
}

impl_251!();