macro_rules! deps {
    () => {
        FilterMap!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < I , P > FilterMap < I , P > { # [doc = " Creates a new `FilterMap` iterator."] pub (super) fn new (base : I , filter_op : P) -> Self { FilterMap { base , filter_op } } }
    };
}

impl_471!()