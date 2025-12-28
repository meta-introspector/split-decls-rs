macro_rules! deps {
    () => {
        Conversion!();
    };
}

macro_rules! parse_generics {
    () => {
        deps!();
        fn parse_generics (decl : & Signature) -> HashMap < Ident , Conversion > { let mut ty_conversions = HashMap :: new () ; for gp in decl . generics . params . iter () { if let GenericParam :: Type (ref tp) = gp { if let Some (conversion) = parse_bounds (& tp . bounds) { ty_conversions . insert (tp . ident . clone () , conversion) ; } } } if let Some (ref wc) = decl . generics . where_clause { for wp in wc . predicates . iter () { if let WherePredicate :: Type (ref pt) = wp { if let Some (ident) = parse_bounded_type (& pt . bounded_ty) { if let Some (conversion) = parse_bounds (& pt . bounds) { ty_conversions . insert (ident , conversion) ; } } } } } ty_conversions }
    };
}

parse_generics!();