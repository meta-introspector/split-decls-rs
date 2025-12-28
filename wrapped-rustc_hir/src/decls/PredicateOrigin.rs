macro_rules! deps {
    () => {
        GenericParam!();
    };
}

macro_rules! PredicateOrigin {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic , PartialEq , Eq)] pub enum PredicateOrigin { WhereClause , GenericParam , ImplTrait , }
    };
}

PredicateOrigin!()