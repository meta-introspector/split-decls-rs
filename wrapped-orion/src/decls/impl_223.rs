macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < const RATE : usize > Debug for Sha3 < RATE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "State {{ state: [***OMITTED***], buffer: [***OMITTED***], capacity: {:?}, leftover: {:?}, \
            is_finalized: {:?} }}" , self . capacity , self . leftover , self . is_finalized) } }
    };
}

impl_223!();