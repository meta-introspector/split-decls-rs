macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > hash :: Hash for VecInner < T , LenT , S > where T : core :: hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { < [T] as hash :: Hash > :: hash (self , state) ; } }
    };
}

impl_299!();