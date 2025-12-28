macro_rules! deps {
    () => {
        MaxScored!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < K : PartialOrd , T > PartialEq for MaxScored < K , T > { # [inline] fn eq (& self , other : & MaxScored < K , T >) -> bool { self . cmp (other) == Ordering :: Equal } }
    };
}

impl_16!();