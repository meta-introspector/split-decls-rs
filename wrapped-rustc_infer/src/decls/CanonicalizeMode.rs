macro_rules! deps {
    () => {
        Canonicalizer!();
    };
}

macro_rules! CanonicalizeMode {
    () => {
        deps!();
        # [doc = " Controls how we canonicalize \"free regions\" that are not inference"] # [doc = " variables. This depends on what we are canonicalizing *for* --"] # [doc = " e.g., if we are canonicalizing to create a query, we want to"] # [doc = " replace those with inference variables, since we want to make a"] # [doc = " maximally general query. But if we are canonicalizing a *query"] # [doc = " response*, then we don't typically replace free regions, as they"] # [doc = " must have been introduced from other parts of the system."] trait CanonicalizeMode { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > ; fn any (& self) -> bool ; fn preserve_universes (& self) -> bool ; }
    };
}

CanonicalizeMode!();