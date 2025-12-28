macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < A , B > Chain < A , B > { # [doc = " Creates a new `Chain` iterator."] pub (super) fn new (a : A , b : B) -> Self { Chain { a , b } } }
    };
}

impl_303!()