macro_rules! deps {
    () => {
        ConstCtOption!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > From < ConstCtOption < T > > for CtOption < T > { # [inline] fn from (value : ConstCtOption < T >) -> Self { CtOption :: new (value . value , value . is_some . into ()) } }
    };
}

impl_61!()