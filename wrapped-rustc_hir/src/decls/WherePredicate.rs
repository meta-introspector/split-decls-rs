macro_rules! deps {
    () => {
        WherePredicateKind!();
    };
}

macro_rules! WherePredicate {
    () => {
        deps!();
        # [doc = " A single predicate in a where-clause."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WherePredicate < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , pub kind : & 'hir WherePredicateKind < 'hir > , }
    };
}

WherePredicate!();