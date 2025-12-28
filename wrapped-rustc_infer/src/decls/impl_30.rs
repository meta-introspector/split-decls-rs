macro_rules! deps {
    () => {
        CanonicalizeMode!();
        CanonicalizeAllFreeRegions!();
        Canonicalizer!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl CanonicalizeMode for CanonicalizeAllFreeRegions { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { canonicalizer . canonical_var_for_region_in_root_universe (r) } fn any (& self) -> bool { true } fn preserve_universes (& self) -> bool { false } }
    };
}

impl_30!()