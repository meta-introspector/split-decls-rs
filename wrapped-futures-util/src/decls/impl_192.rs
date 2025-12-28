macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T > Ready < T > { # [doc = " Unwraps the value from this immediately ready future."] # [inline] pub fn into_inner (mut self) -> T { self . 0 . take () . unwrap () } }
    };
}

impl_192!()