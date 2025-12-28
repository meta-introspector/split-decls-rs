macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PartialOrd < char > for PotentialCodePoint { fn partial_cmp (& self , other : & char) -> Option < Ordering > { self . partial_cmp (& Self :: from_char (* other)) } }
    };
}

impl_8!();