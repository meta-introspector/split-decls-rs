macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < K , V , A : Allocator > IntoIter < K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . iter () , marker : PhantomData , } } }
    };
}

impl_254!()