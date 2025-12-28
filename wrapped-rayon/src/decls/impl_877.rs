macro_rules! deps {
    () => {
        TakeAny!();
    };
}

macro_rules! impl_877 {
    () => {
        deps!();
        impl < I > TakeAny < I > { # [doc = " Creates a new `TakeAny` iterator."] pub (super) fn new (base : I , count : usize) -> Self { TakeAny { base , count } } }
    };
}

impl_877!()