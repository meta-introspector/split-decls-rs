macro_rules! deps {
    () => {
        SmallIndex!();
        SmallIndexError!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl core :: fmt :: Display for SmallIndexError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create small index from {:?}, which exceeds {:?}" , self . attempted () , SmallIndex :: MAX ,) } }
    };
}

impl_423!()