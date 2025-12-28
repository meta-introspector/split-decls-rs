macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < K : Debug , V > fmt :: Debug for Keys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_269!()