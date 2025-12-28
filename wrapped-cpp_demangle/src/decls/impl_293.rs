macro_rules! deps {
    () => {
        Parse!();
        SeqId!();
        IndexStr!();
        Result!();
        Error!();
        ParseContext!();
        Substitution!();
        SubstitutionTable!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl Parse for Substitution { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Substitution , IndexStr < 'b >) > { try_begin_parse ! ("Substitution" , ctx , input) ; if let Ok ((well_known , tail)) = try_recurse ! (WellKnownComponent :: parse (ctx , subs , input)) { return Ok ((Substitution :: WellKnown (well_known) , tail)) ; } let tail = consume (b"S" , input) ? ; let (idx , tail) = if let Ok ((idx , tail)) = try_recurse ! (SeqId :: parse (ctx , subs , tail)) { (idx . 0 + 1 , tail) } else { (0 , tail) } ; if ! subs . contains (idx) { return Err (error :: Error :: BadBackReference) ; } let tail = consume (b"_" , tail) ? ; log ! ("Found a reference to @ {}" , idx) ; Ok ((Substitution :: BackReference (idx) , tail)) } }
    };
}

impl_293!()