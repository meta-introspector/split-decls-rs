macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T , const N : usize > PartialOrd for SmallVec < T , N > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & SmallVec < T , N >) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
    };
}

impl_79!()