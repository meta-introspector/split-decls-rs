macro_rules! deps {
    () => {
        QuickCheck!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Default for QuickCheck { fn default () -> Self { Self :: new () } }
    };
}

impl_75!()