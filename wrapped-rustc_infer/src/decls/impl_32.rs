macro_rules! deps {
    () => {
        CanonicalizeMode!();
        Canonicalizer!();
        CanonicalizeFreeRegionsOtherThanStatic!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl CanonicalizeMode for CanonicalizeFreeRegionsOtherThanStatic { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { if r . is_static () { r } else { canonicalizer . canonical_var_for_region_in_root_universe (r) } } fn any (& self) -> bool { true } fn preserve_universes (& self) -> bool { false } }
    };
}

impl_32!();