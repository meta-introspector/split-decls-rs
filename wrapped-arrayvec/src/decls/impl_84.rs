macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T , const CAP : usize > Ord for ArrayVec < T , CAP > where T : Ord , { fn cmp (& self , other : & Self) -> cmp :: Ordering { (* * self) . cmp (other) } }
    };
}

impl_84!()