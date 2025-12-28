macro_rules! deps {
    () => {
        Value!();
        Null!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < T > From < Option < T > > for Value where T : Into < Self > , { # [inline] fn from (v : Option < T >) -> Self { match v { Some (x) => x . into () , None => Self :: Null , } } }
    };
}

impl_497!();