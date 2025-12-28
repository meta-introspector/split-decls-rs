macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + Debug , V : Debug > Debug for Ref < 'a , K , V > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Ref") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
    };
}

impl_63!()