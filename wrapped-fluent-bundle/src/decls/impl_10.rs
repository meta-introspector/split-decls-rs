macro_rules! deps {
    () => {
        FluentBundle!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < R > Default for FluentBundle < R , IntlLangMemoizer > { fn default () -> Self { Self :: new (vec ! [LanguageIdentifier :: default ()]) } }
    };
}

impl_10!()