macro_rules! deps {
    () => {
        EffectsDisplay!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl core :: fmt :: Display for EffectsDisplay { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { for index in self . 0 . index_iter () { f . write_str (METADATA [index] . escape) ? ; } Ok (()) } }
    };
}

impl_36!();