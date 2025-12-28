macro_rules! deps {
    () => {
        Canonicalizer!();
        CanonicalizeQueryResponse!();
        CanonicalizeMode!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl CanonicalizeMode for CanonicalizeQueryResponse { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , mut r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { let infcx = canonicalizer . infcx . unwrap () ; if let ty :: ReVar (vid) = r . kind () { r = infcx . inner . borrow_mut () . unwrap_region_constraints () . opportunistic_resolve_var (canonicalizer . tcx , vid) ; debug ! ("canonical: region var found with vid {vid:?}, \
                     opportunistically resolved to {r:?}" ,) ; } ; match r . kind () { ty :: ReLateParam (_) | ty :: ReErased | ty :: ReStatic | ty :: ReEarlyParam (..) => r , ty :: RePlaceholder (placeholder) => canonicalizer . canonical_var_for_region (CanonicalVarKind :: PlaceholderRegion (placeholder) , r) , ty :: ReVar (vid) => { let universe = infcx . inner . borrow_mut () . unwrap_region_constraints () . probe_value (vid) . unwrap_err () ; canonicalizer . canonical_var_for_region (CanonicalVarKind :: Region (universe) , r) } _ => { canonicalizer . tcx . dcx () . delayed_bug (format ! ("unexpected region in query response: `{r:?}`")) ; r } } } fn any (& self) -> bool { false } fn preserve_universes (& self) -> bool { true } }
    };
}

impl_26!();