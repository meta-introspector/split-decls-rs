macro_rules! impl_819 {
    () => {
        impl < T > PartialOrd for OrderWrapper < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_819!();