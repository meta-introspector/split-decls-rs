macro_rules! deps {
    () => {
        ToUsize!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl ToUsize for u16 { # [inline] fn to_usize (& self) -> usize { * self as usize } }
    };
}

impl_358!()