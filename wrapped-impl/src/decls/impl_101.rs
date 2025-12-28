macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl PartialOrd for IdentUnraw { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (Self :: cmp (self , other)) } }
    };
}

impl_101!();