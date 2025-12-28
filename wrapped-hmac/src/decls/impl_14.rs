macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < D : EagerHash > MacMarker for HmacResetCore < D > { }
    };
}

impl_14!()