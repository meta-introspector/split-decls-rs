macro_rules! deps {
    () => {
        PredicateOrigin!();
        WherePredicateKind!();
        GenericBounds!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'hir > WherePredicateKind < 'hir > { pub fn in_where_clause (& self) -> bool { match self { WherePredicateKind :: BoundPredicate (p) => p . origin == PredicateOrigin :: WhereClause , WherePredicateKind :: RegionPredicate (p) => p . in_where_clause , WherePredicateKind :: EqPredicate (_) => false , } } pub fn bounds (& self) -> GenericBounds < 'hir > { match self { WherePredicateKind :: BoundPredicate (p) => p . bounds , WherePredicateKind :: RegionPredicate (p) => p . bounds , WherePredicateKind :: EqPredicate (_) => & [] , } } }
    };
}

impl_147!()