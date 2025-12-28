macro_rules! PredicateFilter {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum PredicateFilter { SelfTrait , All , }
    };
}

PredicateFilter!();