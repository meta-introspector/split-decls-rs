macro_rules! deps {
    () => {
        AtomicConsume!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [cfg (not (crossbeam_no_atomic))] impl < T > AtomicConsume for core :: sync :: atomic :: AtomicPtr < T > { type Val = * mut T ; impl_consume ! () ; }
    };
}

impl_67!()