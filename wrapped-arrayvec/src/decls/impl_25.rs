macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < const CAP : usize > PartialOrd < str > for ArrayString < CAP > { fn partial_cmp (& self , rhs : & str) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (rhs) } fn lt (& self , rhs : & str) -> bool { & * * self < rhs } fn le (& self , rhs : & str) -> bool { & * * self <= rhs } fn gt (& self , rhs : & str) -> bool { & * * self > rhs } fn ge (& self , rhs : & str) -> bool { & * * self >= rhs } }
    };
}

impl_25!();