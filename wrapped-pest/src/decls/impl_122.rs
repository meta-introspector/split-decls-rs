macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'i > Ord for Position < 'i > { fn cmp (& self , other : & Position < 'i >) -> Ordering { self . partial_cmp (other) . expect ("cannot compare positions from different strs") } }
    };
}

impl_122!();