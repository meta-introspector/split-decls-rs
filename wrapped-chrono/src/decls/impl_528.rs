macro_rules! deps {
    () => {
        NaiveWeek!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl Hash for NaiveWeek { fn hash < H : Hasher > (& self , state : & mut H) { self . first_day () . hash (state) ; } }
    };
}

impl_528!();