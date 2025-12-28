macro_rules! deps {
    () => {
        Deferred!();
        Bag!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Default for Bag { fn default () -> Self { Self { len : 0 , deferreds : [Deferred :: NO_OP ; MAX_OBJECTS] , } } }
    };
}

impl_104!()