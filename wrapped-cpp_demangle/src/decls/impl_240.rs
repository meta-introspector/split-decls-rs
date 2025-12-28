macro_rules! deps {
    () => {
        MemberName!();
        Result!();
        TemplateArgs!();
        NonSubstitution!();
        Name!();
        ParseContext!();
        Parse!();
        SubstitutionTable!();
        Substitutable!();
        UnscopedTemplateName!();
        UnscopedName!();
        UnqualifiedName!();
        IndexStr!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl Parse for MemberName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (MemberName , IndexStr < 'b >) > { try_begin_parse ! ("MemberName" , ctx , input) ; let (name , tail) = UnqualifiedName :: parse (ctx , subs , input) ? ; let name = UnscopedName :: Unqualified (name) ; if let Ok ((template , tail)) = try_recurse ! (TemplateArgs :: parse (ctx , subs , tail)) { let name = UnscopedTemplateName (name) ; let idx = subs . insert_non_substitution (Substitutable :: UnscopedTemplateName (name)) ; let handle = UnscopedTemplateNameHandle :: NonSubstitution (NonSubstitution (idx)) ; Ok ((MemberName (Name :: UnscopedTemplate (handle , template)) , tail)) } else { Ok ((MemberName (Name :: Unscoped (name)) , tail)) } } }
    };
}

impl_240!();