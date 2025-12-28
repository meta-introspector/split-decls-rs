macro_rules! deps {
    () => {
        Sha256!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Default for Sha256 { fn default () -> Self { Self :: new () } }
    };
}

impl_77!()