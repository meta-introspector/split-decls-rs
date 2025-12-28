macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "wnaf-memuse")] impl < 'a , G : Group + memuse :: DynamicUsage > memuse :: DynamicUsage for Wnaf < usize , Vec < G > , & 'a [i64] > { fn dynamic_usage (& self) -> usize { self . base . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . base . dynamic_usage_bounds () } }
    };
}

impl_32!();