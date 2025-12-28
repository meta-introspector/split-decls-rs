macro_rules! deps {
    () => {
        ConstCtOption!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T > From < ConstCtOption < T > > for Option < T > { # [inline] fn from (value : ConstCtOption < T >) -> Self { if value . is_some . into () { Some (value . value) } else { None } } }
    };
}

impl_62!()