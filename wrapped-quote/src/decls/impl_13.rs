macro_rules! deps {
    () => {
        IdentFragment!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > IdentFragment for Cow < '_ , T > where T : IdentFragment + ToOwned + ? Sized , { fn span (& self) -> Option < Span > { T :: span (self) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { T :: fmt (self , f) } }
    };
}

impl_13!();