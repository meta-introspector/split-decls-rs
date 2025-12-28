macro_rules! deps {
    () => {
        RefMut!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + Debug , V : Debug > Debug for RefMut < 'a , K , V > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("RefMut") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
    };
}

impl_67!()