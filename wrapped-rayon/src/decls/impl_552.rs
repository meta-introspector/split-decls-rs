macro_rules! deps {
    () => {
        Fold!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < I : Debug , ID , F > Debug for Fold < I , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . finish () } }
    };
}

impl_552!();