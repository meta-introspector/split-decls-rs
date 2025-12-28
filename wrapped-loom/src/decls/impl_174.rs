macro_rules! deps {
    () => {
        AccessError!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl fmt :: Display for AccessError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt ("already destroyed" , f) } }
    };
}

impl_174!()