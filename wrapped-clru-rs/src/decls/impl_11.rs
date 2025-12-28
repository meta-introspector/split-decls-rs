macro_rules! deps {
    () => {
        FixedSizeListIter!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for FixedSizeListIter < '_ , T > { fn len (& self) -> usize { self . size_hint () . 0 } }
    };
}

impl_11!();