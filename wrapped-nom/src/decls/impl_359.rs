macro_rules! deps {
    () => {
        ToUsize!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl ToUsize for usize { # [inline] fn to_usize (& self) -> usize { * self } }
    };
}

impl_359!();