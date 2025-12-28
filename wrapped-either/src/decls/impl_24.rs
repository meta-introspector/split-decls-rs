macro_rules! deps {
    () => {
        IterEither!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < L , R > ExactSizeIterator for IterEither < L , R > where L : ExactSizeIterator , R : ExactSizeIterator , { fn len (& self) -> usize { for_both ! (self . inner , ref inner => inner . len ()) } }
    };
}

impl_24!();