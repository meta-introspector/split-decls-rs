macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl PartialEq for Flag < '_ > { fn eq (& self , other : & Flag < '_ >) -> bool { self . cmp (other) == Ordering :: Equal } }
    };
}

impl_355!();