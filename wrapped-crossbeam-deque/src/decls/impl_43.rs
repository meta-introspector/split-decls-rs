macro_rules! deps {
    () => {
        Steal!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Steal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Empty => f . pad ("Empty") , Self :: Success (_) => f . pad ("Success(..)") , Self :: Retry => f . pad ("Retry") , } } }
    };
}

impl_43!()