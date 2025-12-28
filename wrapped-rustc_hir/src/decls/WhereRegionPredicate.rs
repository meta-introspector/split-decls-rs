macro_rules! deps {
    () => {
        GenericBounds!();
        Lifetime!();
    };
}

macro_rules! WhereRegionPredicate {
    () => {
        deps!();
        # [doc = " A lifetime predicate (e.g., `'a: 'b + 'c`)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WhereRegionPredicate < 'hir > { pub in_where_clause : bool , pub lifetime : & 'hir Lifetime , pub bounds : GenericBounds < 'hir > , }
    };
}

WhereRegionPredicate!();