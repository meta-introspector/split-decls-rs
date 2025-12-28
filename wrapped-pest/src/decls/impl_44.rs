macro_rules! deps {
    () => {
        Pair!();
        QueueableToken!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'i , R : Hash > Hash for Pair < 'i , R > { fn hash < H : Hasher > (& self , state : & mut H) { (& * self . queue as * const Vec < QueueableToken < 'i , R > >) . hash (state) ; (self . input as * const str) . hash (state) ; self . start . hash (state) ; } }
    };
}

impl_44!()