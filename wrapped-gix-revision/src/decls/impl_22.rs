macro_rules! deps {
    () => {
        GenThenTime!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl PartialOrd < Self > for GenThenTime { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_22!();