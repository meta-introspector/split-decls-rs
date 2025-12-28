macro_rules! deps {
    () => {
        Iter!();
        Drain!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Drain < '_ , K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . iter () , marker : PhantomData , } } }
    };
}

impl_274!();