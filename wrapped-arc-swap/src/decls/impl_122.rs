macro_rules! deps {
    () => {
        NoFastSlots!();
        Config!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Config for NoFastSlots { const USE_FAST : bool = false ; }
    };
}

impl_122!()