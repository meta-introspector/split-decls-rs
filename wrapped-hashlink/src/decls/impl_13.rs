macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < K : Hash + Eq + Ord , V : Ord , S : BuildHasher > Ord for LinkedHashMap < K , V , S > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other) } }
    };
}

impl_13!();