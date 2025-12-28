macro_rules! deps {
    () => {
        LifetimeKind!();
        Param!();
        WhereRegionPredicate!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'hir > WhereRegionPredicate < 'hir > { # [doc = " Returns `true` if `param_def_id` matches the `lifetime` of this predicate."] fn is_param_bound (& self , param_def_id : LocalDefId) -> bool { self . lifetime . kind == LifetimeKind :: Param (param_def_id) } }
    };
}

impl_152!();