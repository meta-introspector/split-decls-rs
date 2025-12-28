macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_788 {
    () => {
        deps!();
        impl PartialOrd for Time { fn partial_cmp (& self , other : & Time) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_788!()