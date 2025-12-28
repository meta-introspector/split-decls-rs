macro_rules! deps {
    () => {
        MinScored!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < K : PartialOrd , T > Ord for MinScored < K , T > { # [inline] fn cmp (& self , other : & MinScored < K , T >) -> Ordering { let a = & self . 0 ; let b = & other . 0 ; if a == b { Ordering :: Equal } else if a < b { Ordering :: Greater } else if a > b { Ordering :: Less } else if a . ne (a) && b . ne (b) { Ordering :: Equal } else if a . ne (a) { Ordering :: Less } else { Ordering :: Greater } } }
    };
}

impl_14!()