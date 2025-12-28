macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < K , V , A : Allocator > IntoIter < K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub fn rustc_iter (& self) -> Iter < '_ , K , V > { self . iter () } }
    };
}

impl_378!()