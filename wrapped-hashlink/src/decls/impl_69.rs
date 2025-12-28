macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < K , V > fmt :: Debug for IntoIter < K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_69!()