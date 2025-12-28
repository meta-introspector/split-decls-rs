macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl PartialOrd for PotentialCodePoint { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_6!()