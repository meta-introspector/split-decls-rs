macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl PartialOrd for Arg { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_52!()