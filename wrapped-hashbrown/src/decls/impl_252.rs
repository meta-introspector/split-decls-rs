macro_rules! deps {
    () => {
        Iter!();
        IterMut!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < K , V > IterMut < '_ , K , V > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_252!()