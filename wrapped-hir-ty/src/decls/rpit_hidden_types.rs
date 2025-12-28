macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
        ImplTraitIdx!();
    };
}

macro_rules! rpit_hidden_types {
    () => {
        deps!();
        # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub (crate) fn rpit_hidden_types < 'db > (db : & 'db dyn HirDatabase , function : FunctionId ,) -> ArenaMap < ImplTraitIdx < 'db > , EarlyBinder < 'db , Ty < 'db > > > { let infer = db . infer (function . into ()) ; let mut result = ArenaMap :: new () ; for (opaque , hidden_type) in infer . return_position_impl_trait_types (db) { result . insert (opaque , EarlyBinder :: bind (hidden_type)) ; } result . shrink_to_fit () ; result }
    };
}

rpit_hidden_types!()