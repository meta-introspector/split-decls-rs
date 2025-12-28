macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl PartialOrd for Row < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_66!();