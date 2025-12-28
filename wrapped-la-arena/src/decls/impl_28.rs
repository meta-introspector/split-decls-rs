macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > PartialOrd for Idx < T > { fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_28!()