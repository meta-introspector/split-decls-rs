macro_rules! deps {
    () => {
        WhereEqPredicate!();
        Walkable!();
        WhereRegionPredicate!();
        WhereBoundPredicate!();
    };
}

macro_rules! WherePredicateKind {
    () => {
        deps!();
        # [doc = " Predicate kind in where-clause."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum WherePredicateKind { # [doc = " A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`)."] BoundPredicate (WhereBoundPredicate) , # [doc = " A lifetime predicate (e.g., `'a: 'b + 'c`)."] RegionPredicate (WhereRegionPredicate) , # [doc = " An equality predicate (unsupported)."] EqPredicate (WhereEqPredicate) , }
    };
}

WherePredicateKind!()