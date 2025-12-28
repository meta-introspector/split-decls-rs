macro_rules! deps {
    () => {
        DefaultForLevel!();
        Mapping!();
        Trust!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T > Default for Mapping < T > where T : DefaultForLevel , { fn default () -> Self { Mapping { full : T :: default_for_level (Trust :: Full) , reduced : T :: default_for_level (Trust :: Reduced) , } } }
    };
}

impl_4!();