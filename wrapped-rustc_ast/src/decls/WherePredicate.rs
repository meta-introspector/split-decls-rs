macro_rules! deps {
    () => {
        AttrVec!();
        WherePredicateKind!();
        Walkable!();
    };
}

macro_rules! WherePredicate {
    () => {
        deps!();
        # [doc = " A single predicate in a where-clause."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WherePredicate { pub attrs : AttrVec , pub kind : WherePredicateKind , pub id : NodeId , pub span : Span , pub is_placeholder : bool , }
    };
}

WherePredicate!();