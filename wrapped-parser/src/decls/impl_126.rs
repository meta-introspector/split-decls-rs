macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T : PartialOrd > PartialOrd for Positioned < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . node . partial_cmp (& other . node) } }
    };
}

impl_126!();