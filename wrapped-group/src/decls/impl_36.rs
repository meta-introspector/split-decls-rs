macro_rules! deps {
    () => {
        WnafScalar!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "wnaf-memuse")] impl < F : PrimeField , const WINDOW_SIZE : usize > memuse :: DynamicUsage for WnafScalar < F , WINDOW_SIZE > { fn dynamic_usage (& self) -> usize { self . wnaf . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . wnaf . dynamic_usage_bounds () } }
    };
}

impl_36!()