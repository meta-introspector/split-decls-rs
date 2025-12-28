macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (feature = "wnaf-memuse")] impl < G : Group + memuse :: DynamicUsage > memuse :: DynamicUsage for Wnaf < () , Vec < G > , Vec < i64 > > { fn dynamic_usage (& self) -> usize { self . base . dynamic_usage () + self . scalar . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { let (base_lower , base_upper) = self . base . dynamic_usage_bounds () ; let (scalar_lower , scalar_upper) = self . scalar . dynamic_usage_bounds () ; (base_lower + scalar_lower , base_upper . zip (scalar_upper) . map (| (a , b) | a + b) ,) } }
    };
}

impl_27!()