macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < K : Eq + Hash + fmt :: Debug , S : BuildHasher + Clone > fmt :: Debug for DashSet < K , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . inner , f) } }
    };
}

impl_112!();