macro_rules! deps {
    () => {
        Debt!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Default for Debt { fn default () -> Self { Debt (AtomicUsize :: new (Self :: NONE)) } }
    };
}

impl_86!()