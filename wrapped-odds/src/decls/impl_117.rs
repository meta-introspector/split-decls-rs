macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < 'a , A > Clone for Stride < 'a , A > { fn clone (& self) -> Stride < 'a , A > { * self } }
    };
}

impl_117!();