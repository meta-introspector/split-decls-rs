macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl PartialOrd for Flag < '_ > { fn partial_cmp (& self , other : & Flag < '_ >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_356!()