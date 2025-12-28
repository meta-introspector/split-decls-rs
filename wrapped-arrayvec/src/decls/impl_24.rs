macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < const CAP : usize > PartialOrd for ArrayString < CAP > { fn partial_cmp (& self , rhs : & Self) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (& * * rhs) } fn lt (& self , rhs : & Self) -> bool { * * self < * * rhs } fn le (& self , rhs : & Self) -> bool { * * self <= * * rhs } fn gt (& self , rhs : & Self) -> bool { * * self > * * rhs } fn ge (& self , rhs : & Self) -> bool { * * self >= * * rhs } }
    };
}

impl_24!()