macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! impl_1013 {
    () => {
        deps!();
        impl < A , B > Zip < A , B > { # [doc = " Creates a new `Zip` iterator."] pub (super) fn new (a : A , b : B) -> Self { Zip { a , b } } }
    };
}

impl_1013!()