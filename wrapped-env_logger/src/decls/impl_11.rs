macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a , T > From < T > for Env < 'a > where T : Into < Cow < 'a , str > > , { fn from (filter_env : T) -> Self { Env :: default () . filter (filter_env . into ()) } }
    };
}

impl_11!()