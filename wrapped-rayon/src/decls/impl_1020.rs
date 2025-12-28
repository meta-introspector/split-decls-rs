macro_rules! deps {
    () => {
        Zip!();
        ZipEq!();
    };
}

macro_rules! impl_1020 {
    () => {
        deps!();
        impl < A , B > ZipEq < A , B > { # [doc = " Creates a new `ZipEq` iterator."] pub (super) fn new (a : A , b : B) -> Self { ZipEq { zip : super :: Zip :: new (a , b) , } } }
    };
}

impl_1020!()