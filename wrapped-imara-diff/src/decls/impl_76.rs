macro_rules! deps {
    () => {
        UnifiedDiffConfig!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Default for UnifiedDiffConfig { fn default () -> Self { UnifiedDiffConfig { context_len : 3 } } }
    };
}

impl_76!()