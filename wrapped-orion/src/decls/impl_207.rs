macro_rules! deps {
    () => {
        Shake256!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl Default for Shake256 { fn default () -> Self { Self :: new () } }
    };
}

impl_207!()