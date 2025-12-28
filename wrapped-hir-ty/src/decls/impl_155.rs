macro_rules! deps {
    () => {
        Diagnostics!();
        HirDatabase!();
        GenericPredicates!();
        PredicateFilter!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        # [salsa :: tracked] impl < 'db > GenericPredicates < 'db > { # [doc = " Resolve the where clause(s) of an item with generics."] # [doc = ""] # [doc = " Diagnostics are computed only for this item's predicates, not for parents."] # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub fn query_with_diagnostics (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> (GenericPredicates < 'db > , Diagnostics) { generic_predicates_filtered_by (db , def , PredicateFilter :: All , | _ | true) } }
    };
}

impl_155!()