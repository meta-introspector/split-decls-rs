macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T , const N : usize > Ord for SmallVec < T , N > where T : Ord , { # [inline] fn cmp (& self , other : & SmallVec < T , N >) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_150!();