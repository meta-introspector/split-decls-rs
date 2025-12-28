macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < T : Default > Default for Arc < T > { # [track_caller] fn default () -> Arc < T > { Arc :: new (Default :: default ()) } }
    };
}

impl_244!()