macro_rules! deps {
    () => {
        Style!();
        AnsiGenericString!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a , I , S : 'a + ToOwned + ? Sized > From < I > for AnsiGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug , { fn from (input : I) -> AnsiGenericString < 'a , S > { AnsiGenericString { string : input . into () , style : Style :: default () , oscontrol : None , } } }
    };
}

impl_35!();