macro_rules! deps {
    () => {
        Config!();
        NoFastSlots!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Config for NoFastSlots { const USE_FAST : bool = false ; }
    };
}

impl_122!();