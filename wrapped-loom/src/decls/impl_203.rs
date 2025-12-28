macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < T : Default > Default for Cell < T > { # [track_caller] fn default () -> Cell < T > { Cell :: new (T :: default ()) } }
    };
}

impl_203!();