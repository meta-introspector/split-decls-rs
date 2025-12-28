macro_rules! deps {
    () => {
        AtomicConsume!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (crossbeam_loom)] impl < T > AtomicConsume for loom :: sync :: atomic :: AtomicPtr < T > { type Val = * mut T ; impl_consume ! () ; }
    };
}

impl_68!();