macro_rules! deps {
    () => {
        FlatMap!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < I , F > FlatMap < I , F > { # [doc = " Creates a new `FlatMap` iterator."] pub (super) fn new (base : I , map_op : F) -> Self { FlatMap { base , map_op } } }
    };
}

impl_510!();