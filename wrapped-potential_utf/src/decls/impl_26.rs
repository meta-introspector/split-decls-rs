macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl PartialOrd < str > for PotentialUtf8 { fn partial_cmp (& self , other : & str) -> Option < Ordering > { self . partial_cmp (Self :: from_str (other)) } }
    };
}

impl_26!()