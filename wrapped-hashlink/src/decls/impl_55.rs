macro_rules! deps {
    () => {
        Iter!();
        IterMut!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < K , V > IterMut < '_ , K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
    };
}

impl_55!();