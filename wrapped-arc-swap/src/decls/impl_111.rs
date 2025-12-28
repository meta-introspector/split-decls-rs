macro_rules! deps {
    () => {
        DefaultConfig!();
        Config!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl Config for DefaultConfig { const USE_FAST : bool = true ; }
    };
}

impl_111!();