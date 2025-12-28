macro_rules! deps {
    () => {
        ParseContext!();
        Parse!();
        Substitution!();
        Result!();
        Error!();
        TemplateParam!();
        NonSubstitution!();
        SubstitutionTable!();
        Substitutable!();
        Decltype!();
        IndexStr!();
        Prefix!();
        UnqualifiedName!();
        TemplateArgs!();
        DataMemberPrefix!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Parse for PrefixHandle { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (PrefixHandle , IndexStr < 'b >) > { try_begin_parse ! ("PrefixHandle" , ctx , input) ; # [inline] fn save (subs : & mut SubstitutionTable , prefix : Prefix , tail_tail : IndexStr < '_ > ,) -> PrefixHandle { if let Some (b'E') = tail_tail . peek () { let idx = subs . insert_non_substitution (Substitutable :: Prefix (prefix)) ; PrefixHandle :: NonSubstitution (NonSubstitution (idx)) } else { let idx = subs . insert (Substitutable :: Prefix (prefix)) ; PrefixHandle :: BackReference (idx) } } let mut tail = input ; let mut current = None ; loop { try_begin_parse ! ("PrefixHandle iteration" , ctx , tail) ; match tail . peek () { Some (b'E') | None => { if let Some (handle) = current { return Ok ((handle , tail)) ; } else { return Err (error :: Error :: UnexpectedEnd) ; } } Some (b'S') => { let (sub , tail_tail) = Substitution :: parse (ctx , subs , tail) ? ; current = Some (match sub { Substitution :: WellKnown (component) => PrefixHandle :: WellKnown (component) , Substitution :: BackReference (idx) => { PrefixHandle :: BackReference (idx) } }) ; tail = tail_tail ; } Some (b'T') => { let (param , tail_tail) = TemplateParam :: parse (ctx , subs , tail) ? ; current = Some (save (subs , Prefix :: TemplateParam (param) , tail_tail)) ; tail = tail_tail ; } Some (b'D') => { if let Ok ((decltype , tail_tail)) = try_recurse ! (Decltype :: parse (ctx , subs , tail)) { current = Some (save (subs , Prefix :: Decltype (decltype) , tail_tail)) ; tail = tail_tail ; } else { let (name , tail_tail) = UnqualifiedName :: parse (ctx , subs , tail) ? ; let prefix = match current { None => Prefix :: Unqualified (name) , Some (handle) => Prefix :: Nested (handle , name) , } ; current = Some (save (subs , prefix , tail_tail)) ; tail = tail_tail ; } } Some (b'I') if current . is_some () && current . as_ref () . unwrap () . is_template_prefix () => { let (args , tail_tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; let prefix = Prefix :: Template (current . unwrap () , args) ; current = Some (save (subs , prefix , tail_tail)) ; tail = tail_tail ; } Some (c) if UnqualifiedName :: starts_with (c , & tail) => { let (name , tail_tail) = UnqualifiedName :: parse (ctx , subs , tail) ? ; if tail_tail . peek () == Some (b'M') { let prefix = match current { None => Prefix :: Unqualified (name) , Some (current) => { let name = match name { UnqualifiedName :: Source (name , _) => name , UnqualifiedName :: LocalSourceName (name , ..) => name , _ => return Err (error :: Error :: UnexpectedText) , } ; Prefix :: DataMember (current , DataMemberPrefix (name)) } } ; current = Some (save (subs , prefix , tail_tail)) ; tail = consume (b"M" , tail_tail) . unwrap () ; } else { let prefix = match current { None => Prefix :: Unqualified (name) , Some (handle) => Prefix :: Nested (handle , name) , } ; current = Some (save (subs , prefix , tail_tail)) ; tail = tail_tail ; } } Some (_) => { if let Some (handle) = current { return Ok ((handle , tail)) ; } else if tail . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } else { return Err (error :: Error :: UnexpectedText) ; } } } } } }
    };
}

impl_104!();