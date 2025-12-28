macro_rules! deps {
    () => {
        Null!();
        ValueRef!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl < T > From < Option < T > > for ValueRef < '_ > where T : Into < Self > , { # [inline] fn from (s : Option < T >) -> Self { match s { Some (x) => x . into () , None => ValueRef :: Null , } } }
    };
}

impl_508!();