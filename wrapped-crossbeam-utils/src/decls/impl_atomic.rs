macro_rules! deps {
    () => {
        AtomicConsume!();
    };
}

macro_rules! impl_atomic {
    () => {
        deps!();
        macro_rules ! impl_atomic { ($ atomic : ident , $ val : ty) => { # [cfg (not (crossbeam_no_atomic))] impl AtomicConsume for core :: sync :: atomic ::$ atomic { type Val = $ val ; impl_consume ! () ; } # [cfg (crossbeam_loom)] impl AtomicConsume for loom :: sync :: atomic ::$ atomic { type Val = $ val ; impl_consume ! () ; } } ; }
    };
}

impl_atomic!();