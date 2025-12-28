macro_rules! deps {
    () => {
        Suggestions!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Default for Suggestions { fn default () -> Self { Self :: Enabled (vec ! []) } }
    };
}

impl_26!()