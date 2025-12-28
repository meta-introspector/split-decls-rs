macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < T : ComObjectInner + Ord > Ord for ComObject < T > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; < T as Ord > :: cmp (inner_self , other_self) } }
    };
}

impl_122!();