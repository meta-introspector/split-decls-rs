macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < K : Hash + Eq , V : PartialEq , S : BuildHasher > PartialEq for LinkedHashMap < K , V , S > { # [inline] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
    };
}

impl_10!();