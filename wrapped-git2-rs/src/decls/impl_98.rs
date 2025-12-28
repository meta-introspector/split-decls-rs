macro_rules! deps {
    () => {
        CheckoutBuilder!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'cb > Default for CheckoutBuilder < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_98!();