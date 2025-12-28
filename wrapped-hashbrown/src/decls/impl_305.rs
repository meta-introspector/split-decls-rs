macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < K , V > fmt :: Debug for IterMut < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_305!()