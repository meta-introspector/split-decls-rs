macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T : ComObjectInner + PartialOrd > PartialOrd for ComObject < T > { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; < T as PartialOrd > :: partial_cmp (inner_self , other_self) } }
    };
}

impl_121!()