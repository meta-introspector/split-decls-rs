macro_rules! deps {
    () => {
        Vec!();
        Box!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < I > FromIterator < I > for Box < [I] > { # [inline (always)] fn from_iter < T : IntoIterator < Item = I > > (iter : T) -> Self { iter . into_iter () . collect :: < Vec < _ > > () . into_boxed_slice () } }
    };
}

impl_60!();