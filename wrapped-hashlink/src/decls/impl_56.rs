macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < K , V > IntoIter < K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
    };
}

impl_56!()