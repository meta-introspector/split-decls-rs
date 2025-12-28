macro_rules! deps {
    () => {
        RefMap!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T > Default for RefMap < T > { fn default () -> Self { RefMap (Default :: default ()) } }
    };
}

impl_51!()