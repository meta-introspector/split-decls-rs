macro_rules! deps {
    () => {
        MappedRefMut!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + Debug , T : Debug + ? Sized > Debug for MappedRefMut < 'a , K , T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MappedRefMut") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
    };
}

impl_78!()