macro_rules! deps {
    () => {
        EarlyBinder!();
    };
}

macro_rules! GenericPredicates {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct GenericPredicates < 'db > { predicates : EarlyBinder < 'db , Box < [Clause < 'db >] > > , own_predicates_start : u32 , is_trait : bool , parent_is_trait : bool , }
    };
}

GenericPredicates!()