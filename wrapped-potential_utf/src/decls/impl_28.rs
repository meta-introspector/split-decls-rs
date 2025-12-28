macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl PartialOrd < PotentialUtf8 > for str { fn partial_cmp (& self , other : & PotentialUtf8) -> Option < Ordering > { PotentialUtf8 :: from_str (self) . partial_cmp (other) } }
    };
}

impl_28!()