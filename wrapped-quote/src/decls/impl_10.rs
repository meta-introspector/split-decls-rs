macro_rules! deps {
    () => {
        IdentFragment!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : IdentFragment + ? Sized > IdentFragment for & T { fn span (& self) -> Option < Span > { < T as IdentFragment > :: span (* self) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { IdentFragment :: fmt (* self , f) } }
    };
}

impl_10!()