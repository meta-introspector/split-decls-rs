macro_rules! deps {
    () => {
        ToUsize!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl ToUsize for u8 { # [inline] fn to_usize (& self) -> usize { * self as usize } }
    };
}

impl_357!()