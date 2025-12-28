macro_rules! deps {
    () => {
        BoolAsSIMD!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl BoolAsSIMD for bool { # [inline (always)] fn any (self) -> bool { self } }
    };
}

impl_108!();