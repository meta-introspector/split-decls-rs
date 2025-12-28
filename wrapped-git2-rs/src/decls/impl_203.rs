macro_rules! deps {
    () => {
        BlameOptions!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl Default for BlameOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_203!()