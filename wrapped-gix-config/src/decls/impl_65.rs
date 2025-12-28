macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Default for Options < '_ > { fn default () -> Self { Self :: no_follow () } }
    };
}

impl_65!()