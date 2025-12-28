macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl Default for Builder { fn default () -> Builder { Builder :: new () } }
    };
}

impl_15!();