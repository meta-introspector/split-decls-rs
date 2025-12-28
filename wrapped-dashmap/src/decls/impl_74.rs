macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : std :: fmt :: Display + ? Sized > std :: fmt :: Display for MappedRef < 'a , K , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (self . value () , f) } }
    };
}

impl_74!();