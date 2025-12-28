macro_rules! deps {
    () => {
        CargoFeatures!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Default for CargoFeatures { fn default () -> Self { CargoFeatures :: Selected { features : vec ! [] , no_default_features : false } } }
    };
}

impl_46!()