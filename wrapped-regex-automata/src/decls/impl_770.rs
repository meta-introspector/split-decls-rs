macro_rules! deps {
    () => {
        SmallIndexError!();
        SmallIndex!();
    };
}

macro_rules! impl_770 {
    () => {
        deps!();
        impl core :: fmt :: Display for SmallIndexError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create small index from {:?}, which exceeds {:?}" , self . attempted () , SmallIndex :: MAX ,) } }
    };
}

impl_770!()