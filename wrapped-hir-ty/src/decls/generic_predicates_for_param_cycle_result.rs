macro_rules! deps {
    () => {
        EarlyBinder!();
        HirDatabase!();
    };
}

macro_rules! generic_predicates_for_param_cycle_result {
    () => {
        deps!();
        pub (crate) fn generic_predicates_for_param_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _def : GenericDefId , _param_id : TypeOrConstParamId , _assoc_name : Option < Name > ,) -> EarlyBinder < 'db , Box < [Clause < 'db >] > > { EarlyBinder :: bind (Box :: new ([])) }
    };
}

generic_predicates_for_param_cycle_result!()