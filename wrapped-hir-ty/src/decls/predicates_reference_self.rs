macro_rules! deps {
    () => {
        GenericPredicates!();
        AllowSelfProjection!();
        HirDatabase!();
    };
}

macro_rules! predicates_reference_self {
    () => {
        deps!();
        fn predicates_reference_self (db : & dyn HirDatabase , trait_ : TraitId) -> bool { GenericPredicates :: query_explicit (db , trait_ . into ()) . iter_identity_copied () . any (| pred | predicate_references_self (db , trait_ , pred , AllowSelfProjection :: No)) }
    };
}

predicates_reference_self!();