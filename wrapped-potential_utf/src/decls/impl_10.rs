macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl PartialOrd < PotentialCodePoint > for char { fn partial_cmp (& self , other : & PotentialCodePoint) -> Option < Ordering > { PotentialCodePoint :: from_char (* self) . partial_cmp (other) } }
    };
}

impl_10!()