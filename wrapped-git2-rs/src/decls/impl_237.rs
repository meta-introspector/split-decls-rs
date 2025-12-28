macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl Default for Buf { fn default () -> Self { Self :: new () } }
    };
}

impl_237!();