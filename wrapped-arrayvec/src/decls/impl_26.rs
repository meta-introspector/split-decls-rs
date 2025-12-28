macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < const CAP : usize > PartialOrd < ArrayString < CAP > > for str { fn partial_cmp (& self , rhs : & ArrayString < CAP >) -> Option < cmp :: Ordering > { self . partial_cmp (& * * rhs) } fn lt (& self , rhs : & ArrayString < CAP >) -> bool { self < & * * rhs } fn le (& self , rhs : & ArrayString < CAP >) -> bool { self <= & * * rhs } fn gt (& self , rhs : & ArrayString < CAP >) -> bool { self > & * * rhs } fn ge (& self , rhs : & ArrayString < CAP >) -> bool { self >= & * * rhs } }
    };
}

impl_26!();