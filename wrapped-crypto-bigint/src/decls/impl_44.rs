macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > From < Checked < T > > for CtOption < T > { fn from (checked : Checked < T >) -> CtOption < T > { checked . 0 } }
    };
}

impl_44!()