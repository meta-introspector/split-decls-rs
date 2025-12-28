macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , IDX , V > VacantEntry < 'a , IDX , V > { # [doc = " Sets the value of the entry with the `VacantEntry`’s key, and returns a mutable reference to it."] pub fn insert (self , value : V) -> & 'a mut V { self . slot . insert (value) } }
    };
}

impl_16!()