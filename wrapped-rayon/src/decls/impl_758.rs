macro_rules! deps {
    () => {
        IterBridge!();
        ParallelBridge!();
    };
}

macro_rules! impl_758 {
    () => {
        deps!();
        impl < T > ParallelBridge for T where T : Iterator < Item : Send > + Send , { fn par_bridge (self) -> IterBridge < Self > { IterBridge { iter : self } } }
    };
}

impl_758!();