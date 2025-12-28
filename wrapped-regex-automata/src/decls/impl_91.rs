macro_rules! deps {
    () => {
        Slots!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Slots { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "S") ? ; for slot in self . iter () { write ! (f , "-{slot:?}") ? ; } Ok (()) } }
    };
}

impl_91!()