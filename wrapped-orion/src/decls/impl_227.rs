macro_rules! deps {
    () => {
        Shake!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < const RATE : usize > Debug for Shake < RATE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "State {{ state: [***OMITTED***], buffer: [***OMITTED***], capacity: {:?}, until_absorb: {:?}, \
            to_squeeze: {:?}, is_finalized: {:?} }}" , self . capacity , self . until_absorb , self . to_squeeze , self . is_finalized) } }
    };
}

impl_227!();