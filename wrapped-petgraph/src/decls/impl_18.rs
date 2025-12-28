macro_rules! deps {
    () => {
        MaxScored!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < K : PartialOrd , T > PartialOrd for MaxScored < K , T > { # [inline] fn partial_cmp (& self , other : & MaxScored < K , T >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_18!()