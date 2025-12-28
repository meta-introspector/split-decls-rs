macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < T > From < T > for Cell < T > { # [track_caller] fn from (src : T) -> Cell < T > { Cell :: new (src) } }
    };
}

impl_205!()