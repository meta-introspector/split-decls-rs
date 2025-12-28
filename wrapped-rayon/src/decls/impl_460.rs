macro_rules! deps {
    () => {
        Filter!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < I , P > Filter < I , P > { # [doc = " Creates a new `Filter` iterator."] pub (super) fn new (base : I , filter_op : P) -> Self { Filter { base , filter_op } } }
    };
}

impl_460!();