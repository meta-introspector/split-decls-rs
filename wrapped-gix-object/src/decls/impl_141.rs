macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl PartialOrd for EntryRef < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_141!()