macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T : ? Sized + PartialOrd > PartialOrd for Arc < T > { fn partial_cmp (& self , other : & Arc < T >) -> Option < Ordering > { (* * self) . partial_cmp (& * * other) } fn lt (& self , other : & Arc < T >) -> bool { * (* self) < * (* other) } fn le (& self , other : & Arc < T >) -> bool { * (* self) <= * (* other) } fn gt (& self , other : & Arc < T >) -> bool { * (* self) > * (* other) } fn ge (& self , other : & Arc < T >) -> bool { * (* self) >= * (* other) } }
    };
}

impl_142!();