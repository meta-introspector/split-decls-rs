macro_rules! deps {
    () => {
        Pointable!();
        Shared!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Ord for Shared < '_ , T > { fn cmp (& self , other : & Self) -> cmp :: Ordering { self . data . cmp (& other . data) } }
    };
}

impl_60!()