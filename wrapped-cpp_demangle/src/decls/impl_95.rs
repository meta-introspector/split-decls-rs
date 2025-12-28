macro_rules! deps {
    () => {
        CvQualifiers!();
        Result!();
        Parse!();
        Error!();
        IndexStr!();
        SubstitutionTable!();
        ParseContext!();
        NonSubstitution!();
        Substitutable!();
        Prefix!();
        NestedName!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Parse for NestedName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (NestedName , IndexStr < 'b >) > { try_begin_parse ! ("NestedName" , ctx , input) ; let tail = consume (b"N" , input) ? ; let (cv_qualifiers , ref_qualifier , explicit_obj_param , tail) = match tail . peek () { Some (b'H') => { let (explicit_obj_param , tail) = ExplicitObjectParameter :: parse (ctx , subs , tail) ? ; (Default :: default () , None , Some (explicit_obj_param) , tail) } _ => { let (cv_qualifiers , tail) = if let Ok ((q , tail)) = try_recurse ! (CvQualifiers :: parse (ctx , subs , tail)) { (q , tail) } else { (Default :: default () , tail) } ; let (ref_qualifier , tail) = if let Ok ((r , tail)) = try_recurse ! (RefQualifier :: parse (ctx , subs , tail)) { (Some (r) , tail) } else { (None , tail) } ; (cv_qualifiers , ref_qualifier , None , tail) } } ; let (prefix , tail) = PrefixHandle :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let substitutable = match prefix { PrefixHandle :: BackReference (idx) => subs . get (idx) , PrefixHandle :: NonSubstitution (NonSubstitution (idx)) => subs . get_non_substitution (idx) , PrefixHandle :: WellKnown (_) => None , } ; match (substitutable , explicit_obj_param) { (Some (& Substitutable :: Prefix (Prefix :: Unqualified (ref name))) , None) => Ok ((NestedName :: Unqualified (cv_qualifiers , ref_qualifier , None , name . clone ()) , tail ,)) , (Some (& Substitutable :: Prefix (Prefix :: Nested (ref prefix , ref name))) , None) => Ok ((NestedName :: Unqualified (cv_qualifiers , ref_qualifier , Some (prefix . clone ()) , name . clone () ,) , tail ,)) , (Some (& Substitutable :: Prefix (Prefix :: Template (..))) , None) => Ok ((NestedName :: Template (cv_qualifiers , ref_qualifier , prefix) , tail ,)) , (Some (& Substitutable :: Prefix (Prefix :: Unqualified (ref name))) , Some (param)) => Ok ((NestedName :: UnqualifiedExplicitObject (None , name . clone () , param) , tail ,)) , (Some (& Substitutable :: Prefix (Prefix :: Nested (ref prefix , ref name))) , Some (param)) => { Ok ((NestedName :: UnqualifiedExplicitObject (Some (prefix . clone ()) , name . clone () , param ,) , tail ,)) } (Some (& Substitutable :: Prefix (Prefix :: Template (..))) , Some (param)) => { Ok ((NestedName :: TemplateExplicitObject (prefix , param) , tail)) } _ => Err (error :: Error :: UnexpectedText) , } } }
    };
}

impl_95!()