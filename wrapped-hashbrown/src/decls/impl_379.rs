macro_rules! deps {
    () => {
        Drain!();
        Iter!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Drain < '_ , K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub fn rustc_iter (& self) -> Iter < '_ , K , V > { self . iter () } }
    };
}

impl_379!();