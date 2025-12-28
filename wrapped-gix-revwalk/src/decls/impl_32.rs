macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < K : Ord , T > Ord for Item < K , T > { fn cmp (& self , other : & Self) -> Ordering { self . key . cmp (& other . key) } }
    };
}

impl_32!();