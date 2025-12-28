macro_rules! deps {
    () => {
        HashMapCache!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl Default for HashMapCache < RandomState > { fn default () -> Self { Self { _mark : PhantomData } } }
    };
}

impl_368!()