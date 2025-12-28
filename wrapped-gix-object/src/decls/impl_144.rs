macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl PartialOrd for Entry { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_144!()