macro_rules! deps {
    () => {
        Result!();
        ParseContext!();
        LocalName!();
        SubstitutionTable!();
        Parse!();
        NestedName!();
        Name!();
        TemplateArgs!();
        IndexStr!();
        Substitutable!();
        UnscopedTemplateName!();
        UnscopedName!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Parse for Name { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Name , IndexStr < 'b >) > { try_begin_parse ! ("Name" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (NestedName :: parse (ctx , subs , input)) { return Ok ((Name :: Nested (name) , tail)) ; } if let Ok ((name , tail)) = try_recurse ! (UnscopedName :: parse (ctx , subs , input)) { if tail . peek () == Some (b'I') { let name = UnscopedTemplateName (name) ; let idx = subs . insert (Substitutable :: UnscopedTemplateName (name)) ; let handle = UnscopedTemplateNameHandle :: BackReference (idx) ; let (args , tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; return Ok ((Name :: UnscopedTemplate (handle , args) , tail)) ; } else { return Ok ((Name :: Unscoped (name) , tail)) ; } } if let Ok ((name , tail)) = try_recurse ! (UnscopedTemplateNameHandle :: parse (ctx , subs , input)) { let (args , tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; return Ok ((Name :: UnscopedTemplate (name , args) , tail)) ; } let (name , tail) = LocalName :: parse (ctx , subs , input) ? ; Ok ((Name :: Local (name) , tail)) } }
    };
}

impl_79!();