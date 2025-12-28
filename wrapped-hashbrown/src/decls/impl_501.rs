macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IterMut < '_ , T > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_501!();