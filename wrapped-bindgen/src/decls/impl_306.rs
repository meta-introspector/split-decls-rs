macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl PartialOrd for Interface { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_306!();