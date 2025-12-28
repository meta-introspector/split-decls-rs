macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > Ord for Idx < T > { fn cmp (& self , other : & Self) -> cmp :: Ordering { self . raw . cmp (& other . raw) } }
    };
}

impl_27!()