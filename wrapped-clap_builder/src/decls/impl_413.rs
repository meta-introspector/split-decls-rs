macro_rules! deps {
    () => {
        Error!();
        ErrorFormatter!();
        Result!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < F : ErrorFormatter > Debug for Error < F > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { self . inner . fmt (f) } }
    };
}

impl_413!()