macro_rules! deps {
    () => {
        Iter!();
        Drain!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < K , V > Drain < '_ , K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
    };
}

impl_57!()