macro_rules! deps {
    () => {
        Immediate!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < Prov : Provenance > From < Scalar < Prov > > for Immediate < Prov > { # [inline (always)] fn from (val : Scalar < Prov >) -> Self { Immediate :: Scalar (val) } }
    };
}

impl_264!()