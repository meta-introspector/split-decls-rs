macro_rules! deps {
    () => {
        MinScored!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < K : PartialOrd , T > PartialEq for MinScored < K , T > { # [inline] fn eq (& self , other : & MinScored < K , T >) -> bool { self . cmp (other) == Ordering :: Equal } }
    };
}

impl_11!()