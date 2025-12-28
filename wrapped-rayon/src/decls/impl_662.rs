macro_rules! deps {
    () => {
        MinLen!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl < I > MinLen < I > { # [doc = " Creates a new `MinLen` iterator."] pub (super) fn new (base : I , min : usize) -> Self { MinLen { base , min } } }
    };
}

impl_662!()