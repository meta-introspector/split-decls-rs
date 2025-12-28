macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < L , R > ExactSizeIterator for Either < L , R > where L : ExactSizeIterator , R : ExactSizeIterator < Item = L :: Item > , { fn len (& self) -> usize { for_both ! (self , inner => inner . len ()) } }
    };
}

impl_20!()