macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T : Ord > Ord for Positioned < T > { fn cmp (& self , other : & Self) -> Ordering { self . node . cmp (& other . node) } }
    };
}

impl_127!();