macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl PartialOrd for Oid { fn partial_cmp (& self , other : & Oid) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_526!()