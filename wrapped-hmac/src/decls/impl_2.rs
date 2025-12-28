macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < D : EagerHash > MacMarker for HmacCore < D > { }
    };
}

impl_2!();