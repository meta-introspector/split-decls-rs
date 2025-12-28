macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < T > From < T > for Arc < T > { # [track_caller] fn from (t : T) -> Self { Arc :: new (t) } }
    };
}

impl_245!()