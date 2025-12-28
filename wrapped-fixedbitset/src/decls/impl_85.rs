macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl PartialOrd for FixedBitSet { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_85!();