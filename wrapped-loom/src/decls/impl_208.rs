macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < T : PartialOrd + Copy > PartialOrd for Cell < T > { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . get () . partial_cmp (& other . get ()) } }
    };
}

impl_208!();