macro_rules! deps {
    () => {
        Pairs!();
        QueueableToken!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'i , R : Hash > Hash for Pairs < 'i , R > { fn hash < H : Hasher > (& self , state : & mut H) { (& * self . queue as * const Vec < QueueableToken < 'i , R > >) . hash (state) ; (self . input as * const str) . hash (state) ; self . start . hash (state) ; self . end . hash (state) ; } }
    };
}

impl_58!();