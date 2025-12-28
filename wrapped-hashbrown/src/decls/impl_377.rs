macro_rules! deps {
    () => {
        IterMut!();
        Iter!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < K , V > IterMut < '_ , K , V > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub fn rustc_iter (& self) -> Iter < '_ , K , V > { self . iter () } }
    };
}

impl_377!()