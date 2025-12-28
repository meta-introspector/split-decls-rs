macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T > From < Checked < T > > for Option < T > { fn from (checked : Checked < T >) -> Option < T > { checked . 0 . into () } }
    };
}

impl_46!()