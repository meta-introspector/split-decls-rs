macro_rules! deps {
    () => {
        MaxLen!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl < I > MaxLen < I > { # [doc = " Creates a new `MaxLen` iterator."] pub (super) fn new (base : I , max : usize) -> Self { MaxLen { base , max } } }
    };
}

impl_668!();