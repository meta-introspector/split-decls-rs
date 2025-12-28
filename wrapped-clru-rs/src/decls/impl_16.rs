macro_rules! deps {
    () => {
        FixedSizeListIterMut!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for FixedSizeListIterMut < '_ , T > { fn len (& self) -> usize { self . size_hint () . 0 } }
    };
}

impl_16!();