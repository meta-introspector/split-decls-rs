macro_rules! deps {
    () => {
        ParUnion!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T , S1 , S2 > fmt :: Debug for ParUnion < '_ , T , S1 , S2 > where T : fmt :: Debug + Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . set1 . union (self . set2)) . finish () } }
    };
}

impl_177!();