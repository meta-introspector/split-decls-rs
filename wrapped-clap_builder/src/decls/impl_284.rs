macro_rules! deps {
    () => {
        EnumValueParser!();
        ValueEnum!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < E : crate :: ValueEnum + Clone + Send + Sync + 'static > Default for EnumValueParser < E > { fn default () -> Self { Self :: new () } }
    };
}

impl_284!();