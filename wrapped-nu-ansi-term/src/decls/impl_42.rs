macro_rules! deps {
    () => {
        Style!();
        AnsiGenericString!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Style { # [doc = " Paints the given text with this color, returning an ANSI string."] # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (self , input : I) -> AnsiGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug , { AnsiGenericString { string : input . into () , style : self , oscontrol : None , } } }
    };
}

impl_42!()