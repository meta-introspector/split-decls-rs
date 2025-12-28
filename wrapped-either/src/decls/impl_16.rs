macro_rules! deps {
    () => {
        IterEither!();
        Either!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < L , R > IterEither < L , R > { pub (crate) fn new (inner : Either < L , R >) -> Self { IterEither { inner } } }
    };
}

impl_16!();