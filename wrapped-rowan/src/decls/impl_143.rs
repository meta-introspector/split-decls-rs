macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T : ? Sized + Ord > Ord for Arc < T > { fn cmp (& self , other : & Arc < T >) -> Ordering { (* * self) . cmp (& * * other) } }
    };
}

impl_143!();