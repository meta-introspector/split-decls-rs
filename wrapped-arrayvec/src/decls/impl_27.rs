macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < const CAP : usize > Ord for ArrayString < CAP > { fn cmp (& self , rhs : & Self) -> cmp :: Ordering { (* * self) . cmp (& * * rhs) } }
    };
}

impl_27!()