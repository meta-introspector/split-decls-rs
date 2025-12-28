macro_rules! fix_elipses {
    () => {
        # [doc = " If the mocked signature contains any variadic parts, they need a pattern."] # [doc = " The pattern is required for the signature of the mock function, a Rust"] # [doc = " function, even though it's not required in the signature of the foreign"] # [doc = " function."] fn fix_elipses (sig : & mut Signature) { if let Some (variadic) = & mut sig . variadic { if variadic . pat . is_none () { let pat = PatIdent { attrs : vec ! [] , by_ref : None , mutability : None , ident : format_ident ! ("_") , subpat : None } ; let colon = Token ! [:] (variadic . span ()) ; variadic . pat = Some ((Box :: new (Pat :: Ident (pat)) , colon)) ; } } }
    };
}

fix_elipses!()