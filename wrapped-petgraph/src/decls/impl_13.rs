macro_rules! deps {
    () => {
        MinScored!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < K : PartialOrd , T > PartialOrd for MinScored < K , T > { # [inline] fn partial_cmp (& self , other : & MinScored < K , T >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_13!();