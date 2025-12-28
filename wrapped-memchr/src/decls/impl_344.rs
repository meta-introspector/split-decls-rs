macro_rules! deps {
    () => {
        PrefilterConfig!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl Default for PrefilterConfig { fn default () -> PrefilterConfig { PrefilterConfig :: Auto } }
    };
}

impl_344!()