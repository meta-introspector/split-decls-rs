macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "wnaf-memuse")] impl < 'a , G : Group > memuse :: DynamicUsage for Wnaf < usize , & 'a [G] , Vec < i64 > > { fn dynamic_usage (& self) -> usize { self . scalar . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . scalar . dynamic_usage_bounds () } }
    };
}

impl_30!();