macro_rules! deps {
    () => {
        WnafBase!();
        Group!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [cfg (feature = "wnaf-memuse")] impl < G : Group + memuse :: DynamicUsage , const WINDOW_SIZE : usize > memuse :: DynamicUsage for WnafBase < G , WINDOW_SIZE > { fn dynamic_usage (& self) -> usize { self . table . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . table . dynamic_usage_bounds () } }
    };
}

impl_39!();