macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + Debug , T : Debug + ? Sized > Debug for MappedRef < 'a , K , T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MappedRef") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
    };
}

impl_72!();