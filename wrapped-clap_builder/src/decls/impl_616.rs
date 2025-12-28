macro_rules! deps {
    () => {
        FlatMap!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl < K : PartialEq + Eq , V > Default for FlatMap < K , V > { fn default () -> Self { Self { keys : Default :: default () , values : Default :: default () , } } }
    };
}

impl_616!()