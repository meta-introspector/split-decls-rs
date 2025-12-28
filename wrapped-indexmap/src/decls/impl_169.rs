macro_rules! deps {
    () => {
        ParIntersection!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T , S1 , S2 > fmt :: Debug for ParIntersection < '_ , T , S1 , S2 > where T : fmt :: Debug + Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . set1 . intersection (self . set2)) . finish () } }
    };
}

impl_169!()