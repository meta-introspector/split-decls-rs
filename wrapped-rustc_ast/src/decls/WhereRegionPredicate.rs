macro_rules! deps {
    () => {
        GenericBounds!();
        LifetimeCtxt!();
        Walkable!();
        BoundKind!();
        Lifetime!();
    };
}

macro_rules! WhereRegionPredicate {
    () => {
        deps!();
        # [doc = " A lifetime predicate."] # [doc = ""] # [doc = " E.g., `'a: 'b + 'c`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WhereRegionPredicate { # [visitable (extra = LifetimeCtxt :: Bound)] pub lifetime : Lifetime , # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , }
    };
}

WhereRegionPredicate!()