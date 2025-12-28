macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > From < CtOption < T > > for Checked < T > { fn from (ct_option : CtOption < T >) -> Checked < T > { Checked (ct_option) } }
    };
}

impl_45!();