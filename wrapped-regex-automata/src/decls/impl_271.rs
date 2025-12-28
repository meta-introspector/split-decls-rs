macro_rules! deps {
    () => {
        LazyStateID!();
        LazyStateIDError!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl core :: fmt :: Display for LazyStateIDError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create LazyStateID from {:?}, which exceeds {:?}" , self . attempted () , LazyStateID :: MAX ,) } }
    };
}

impl_271!()